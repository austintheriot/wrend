use super::{program_link::ProgramLink, shader_type::ShaderType};
use std::fmt::Debug;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};
use thiserror::Error;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Renderer<I>
where
    I: Hash + Eq + Clone + Debug + Default,
{
    canvas: HtmlCanvasElement,
    gl: WebGl2RenderingContext,
    fragment_shaders: HashMap<I, WebGlShader>,
    vertex_shaders: HashMap<I, WebGlShader>,
    programs: HashMap<ProgramLink<I>, WebGlProgram>,
}

impl<I> Renderer<I>
where
    I: Hash + Eq + Clone + Debug + Default,
{
    pub fn builder() -> RendererBuilder<I> {
        RendererBuilder::default()
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct RendererBuilder<I>
where
    I: Hash + Eq + Clone + Debug + Default,
{
    canvas: Option<HtmlCanvasElement>,
    gl: Option<WebGl2RenderingContext>,
    vertex_shader_sources: HashMap<I, String>,
    fragment_shader_sources: HashMap<I, String>,
    vertex_shaders: HashMap<I, WebGlShader>,
    fragment_shaders: HashMap<I, WebGlShader>,
    program_ids_to_link: HashSet<ProgramLink<I>>,
    programs: HashMap<ProgramLink<I>, WebGlProgram>,
}

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum RendererBuilderError {
    // @todo: move this into its own sub-error
    #[error(
        "Error occurred while trying to get a WebGL2 rendering context from the supplied canvas"
    )]
    WebGL2ContextRetrievalError,
    #[error("WebGL2 rendering context could not be acquired from the canvas. The returned value was `None`")]
    WebGL2ContextNotFoundError,
    #[error("The JavaScript Object returned from get_context could not be converted into a `WebGl2RenderingContext`")]
    WebGL2TypeConversionError,

    // @todo: move this into its own sub-error
    #[error("Renderer could not be built with canvas, because no canvas was supplied")]
    NoCanvasBuildError,
    #[error(
        "Renderer could not be built with WebGL2RenderingContext, because no canvas was supplied"
    )]
    NoContextBuildError,

    // @todo: move this into its own sub-error
    #[error("Could not compile shader, because no canvas or its associated context were supplied")]
    NoContextCompileShaderError,
    #[error("Could not compile shader, because call to WebGL2RenderingContext returned None")]
    NoShaderReturnedCompilerShaderError,
    #[error("Could not compile shader. Reason: {0}")]
    KnownErrorCompileShaderError(String),
    #[error("Could not compile shader. An unknown error occurred.")]
    UnknownErrorCompilerShaderError,

    // @todo: move this into its own sub-error
    #[error("Could not link program because no WebGL2RenderingContext was provided")]
    NoContextLinkProgramError,
    #[error(
        "Could not link program because no vertex shader was found associated with the id provided"
    )]
    VertexShaderNotFoundLinkProgramError,
    #[error("Could not link program because no fragment shader was found associated with the id provided")]
    FragmentShaderNotFoundLinkProgramError,
    #[error("Could not link program because value returned by `gl.link_program` was `None`")]
    NoProgramLinkProgramError,
    #[error("Could not link program. Reason: {0}")]
    KnownErrorLinkProgramError(String),
    #[error("Could not link program. An unknown error occurred.")]
    UnknownErrorLinkProgramError,
}

/// Public API
impl<I> RendererBuilder<I>
where
    I: Hash + Eq + Clone + Debug + Default,
{
    /// Save the canvas that will be rendered to and get its associated WebGL2 rendering context
    pub fn set_canvas(
        &mut self,
        canvas: HtmlCanvasElement,
    ) -> Result<&mut Self, RendererBuilderError> {
        let gl = Self::context(&canvas)?;

        self.gl = Some(gl);
        self.canvas = Some(canvas);

        Ok(self)
    }

    /// Saves a fragment shader source and its corresponding id
    pub fn add_fragment_shader_src(&mut self, id: I, fragment_shader_src: String) -> &mut Self {
        self.fragment_shader_sources.insert(id, fragment_shader_src);

        self
    }

    /// Saves a vertex shader source and its corresponding id
    pub fn add_vertex_shader_src(&mut self, id: I, vertex_shader_src: String) -> &mut Self {
        self.vertex_shader_sources.insert(id, vertex_shader_src);

        self
    }

    /// Saves a link between a vertex shader id and a fragment shader id.
    ///
    /// During the Renderer build process, this `program_link` is used to link a new WebGL2 program
    /// together by associating the vertex shader id and the fragment shader id with their corresponding compiled shaders.
    pub fn add_program_link(&mut self, program_link: impl Into<ProgramLink<I>>) -> &mut Self {
        self.program_ids_to_link.insert(program_link.into());

        self
    }

    /// Compiles all vertex shaders and fragment shaders.
    /// Links together any programs that have been specified.
    /// Outputs the final Renderer.
    pub fn build(mut self) -> Result<Renderer<I>, RendererBuilderError> {
        self.compile_fragment_shaders()?;
        self.compile_vertex_shaders()?;
        self.link_programs()?;

        let renderer = Renderer {
            canvas: self
                .canvas
                .ok_or(RendererBuilderError::NoCanvasBuildError)?,
            gl: self.gl.ok_or(RendererBuilderError::NoContextBuildError)?,
            fragment_shaders: self.fragment_shaders,
            vertex_shaders: self.vertex_shaders,
            programs: self.programs,
        };

        Ok(renderer)
    }
}


/// Private API
impl<I> RendererBuilder<I>
where
    I: Hash + Eq + Clone + Debug + Default,
{
    /// Get the WebGL2 rendering context from a canvas
    fn context(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext, RendererBuilderError> {
        let gl = canvas
            .get_context("webgl2")
            .map_err(|_| RendererBuilderError::WebGL2ContextRetrievalError)?;

        let gl = gl.ok_or(RendererBuilderError::WebGL2ContextNotFoundError)?;

        let gl: WebGl2RenderingContext = gl
            .dyn_into()
            .map_err(|_| RendererBuilderError::WebGL2TypeConversionError)?;

        Ok(gl)
    }

    /// Takes the list of fragment shader sources and their ids and saves compiled `WebGlShader`s to state
    fn compile_fragment_shaders(&mut self) -> Result<&mut Self, RendererBuilderError> {
        for (id, fragment_shader_src) in self.fragment_shader_sources.iter() {
            let fragment_shader =
                self.compile_shader(ShaderType::FragmentShader, &fragment_shader_src)?;
            self.fragment_shaders.insert((*id).clone(), fragment_shader);
        }

        Ok(self)
    }

    /// Takes the list of vertex shader sources and their ids and saves compiled `WebGlShader`s to state
    fn compile_vertex_shaders(&mut self) -> Result<&mut Self, RendererBuilderError> {
        for (id, vertex_shader_src) in self.vertex_shader_sources.iter() {
            let vertex_shader =
                self.compile_shader(ShaderType::VertexShader, &vertex_shader_src)?;
            self.vertex_shaders.insert((*id).clone(), vertex_shader);
        }

        Ok(self)
    }

    /// Links together all of the vertex & fragment shaders that have been saved
    /// according to any ProgramLinks that were provided.
    ///
    /// If a ProgramLink does not correspond to an actual shader, returns an Error.
    fn link_programs(&mut self) -> Result<&mut Self, RendererBuilderError> {
        for program_link in self.program_ids_to_link.iter() {
            let vertex_shader_id = program_link.vertex_shader_id();
            let vertex_shader = self
                .vertex_shaders
                .get(vertex_shader_id)
                .ok_or(RendererBuilderError::VertexShaderNotFoundLinkProgramError)?;

            let fragment_shader_id = program_link.fragment_shader_id();
            let fragment_shader = self
                .fragment_shaders
                .get(fragment_shader_id)
                .ok_or(RendererBuilderError::FragmentShaderNotFoundLinkProgramError)?;

            let program = self.link_program(vertex_shader, fragment_shader)?;
            self.programs.insert((*program_link).clone(), program);
        }

        Ok(self)
    }

    fn link_program(
        &self,
        vertex_shader: &WebGlShader,
        fragment_shader: &WebGlShader,
    ) -> Result<WebGlProgram, RendererBuilderError> {
        let gl = self
            .gl
            .as_ref()
            .ok_or(RendererBuilderError::NoContextLinkProgramError)?;

        let program = gl
            .create_program()
            .ok_or(RendererBuilderError::NoProgramLinkProgramError)?;

        gl.attach_shader(&program, vertex_shader);
        gl.attach_shader(&program, fragment_shader);
        gl.link_program(&program);

        if gl
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(match gl.get_program_info_log(&program) {
                Some(known_error) => RendererBuilderError::KnownErrorLinkProgramError(known_error),
                None => RendererBuilderError::UnknownErrorLinkProgramError,
            })
        }
    }

    /// Takes the string source of a shader and compiles to using the current WebGL2RenderingContext
    fn compile_shader(
        &self,
        shader_type: ShaderType,
        source: &str,
    ) -> Result<WebGlShader, RendererBuilderError> {
        let gl = self
            .gl
            .as_ref()
            .ok_or(RendererBuilderError::NoContextCompileShaderError)?;

        let shader = gl
            .create_shader(shader_type.into())
            .ok_or(RendererBuilderError::NoShaderReturnedCompilerShaderError)?;

        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);

        if gl
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(match gl.get_shader_info_log(&shader) {
                Some(known_error) => {
                    RendererBuilderError::KnownErrorCompileShaderError(known_error)
                }
                None => RendererBuilderError::UnknownErrorCompilerShaderError,
            })
        }
    }
}
