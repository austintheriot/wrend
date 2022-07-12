use super::attribute_location::AttributeLocation;
use super::buffer::Buffer;
use super::buffer_link::BufferLink;
use super::default_id::DefaultId;
use super::id::Id;
use super::id_name::IdName;
use super::render_callback::RenderCallback;
use super::uniform::Uniform;
use super::uniform_link::UniformLink;
use super::{program_link::ProgramLink, shader_type::ShaderType};
use js_sys::Date;
use std::fmt::Debug;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};
use thiserror::Error;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Renderer<
    VertexShaderId: Id = DefaultId,
    FragmentShaderId: Id = DefaultId,
    ProgramId: Id = DefaultId,
    UniformId: Id + IdName = DefaultId,
    BufferId: Id + IdName = DefaultId,
    UserCtx = (),
> {
    canvas: HtmlCanvasElement,
    gl: WebGl2RenderingContext,
    fragment_shaders: HashMap<FragmentShaderId, WebGlShader>,
    vertex_shaders: HashMap<VertexShaderId, WebGlShader>,
    programs: HashMap<ProgramId, WebGlProgram>,
    render_callback:
        RenderCallback<VertexShaderId, FragmentShaderId, ProgramId, UniformId, BufferId, UserCtx>,
    uniforms: HashSet<Uniform<ProgramId, UniformId, UserCtx>>,
    user_ctx: Option<UserCtx>,
    buffers: HashSet<Buffer<ProgramId, BufferId, UserCtx>>,
}

/// Public API
impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id + IdName,
        UserCtx,
    > Renderer<VertexShaderId, FragmentShaderId, ProgramId, UniformId, BufferId, UserCtx>
{
    pub fn builder(
    ) -> RendererBuilder<VertexShaderId, FragmentShaderId, ProgramId, UniformId, BufferId, UserCtx>
    {
        RendererBuilder::default()
    }

    pub fn canvas(&self) -> &HtmlCanvasElement {
        &self.canvas
    }

    pub fn gl(&self) -> &WebGl2RenderingContext {
        &self.gl
    }

    pub fn fragment_shaders(&self) -> &HashMap<FragmentShaderId, WebGlShader> {
        &self.fragment_shaders
    }

    pub fn vertex_shaders(&self) -> &HashMap<VertexShaderId, WebGlShader> {
        &self.vertex_shaders
    }

    pub fn programs(&self) -> &HashMap<ProgramId, WebGlProgram> {
        &self.programs
    }

    // @todo - enable ctx to be returned unconditionally (depending on if it's set or not)
    pub fn user_ctx(&self) -> Option<&UserCtx> {
        self.user_ctx.as_ref()
    }

    /// Iterates through all saved uniforms and updates them using their associated update callbacks
    ///
    /// Automatically calls "use_program" on the appropriate program before each uniform's update function
    /// (so this is not necessary to do within the callback itself, unless you need to change programs, for
    /// whatever reason).
    pub fn update_uniforms(&self) -> &Self {
        let now = Date::now();
        let user_ctx = self.user_ctx();
        let gl = self.gl();

        for uniform in &self.uniforms {
            let program_id = uniform.program_id();
            let program = self.programs().get(program_id);
            gl.use_program(program);

            uniform.update(gl, now, user_ctx);

            gl.use_program(None);
        }

        self
    }

    /// Iterates through all saved buffers and updates them using their associated update callbacks
    ///
    /// Automatically calls "use_program" on the appropriate program before each uniform's update function
    /// (so this is not necessary to do within the callback itself, unless you need to change programs, for
    /// whatever reason).
    ///
    /// Also automatically binds the correct buffer before the update callback is called, so this may be omitted.
    pub fn update_buffers(&self) -> &Self {
        let now = Date::now();
        let user_ctx = self.user_ctx();
        let gl = self.gl();

        for buffer in &self.buffers {
            // bind the corresponding program
            let program_id = buffer.program_id();
            let program = self.programs().get(program_id);
            self.gl().use_program(program);

            // bind the corresponding buffer
            let webgl_buffer = buffer.webgl_buffer();
            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(webgl_buffer));

            if buffer.should_update(gl, now, user_ctx) {
                buffer.update(self.gl(), now, user_ctx);
            }

            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
        }

        self
    }

    pub fn render(&self) {
        self.render_callback.call(self);
    }
}

/// Private API
impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id + IdName,
        UserCtx,
    > Renderer<VertexShaderId, FragmentShaderId, ProgramId, UniformId, BufferId, UserCtx>
{
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
    #[error("Renderer could not be built, because no `RenderCallback` was supplied")]
    NoRenderCallbackBuildError,

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

    // @todo: move this into its own sub-error
    #[error("Could not build uniform because the uniform's location was not found in the program")]
    UniformLocationNotFoundLinkBuildUniformError,

    // @todo: move this into its own sub-error
    #[error("Could not build uniform because no WebGL2RenderingContext was provided")]
    NoContextLinkBuildUniformError,
    #[error("Could not build uniform because the associated program_id could no be found")]
    ProgramNotFoundBuildUniformsError,

    // @todo: move this into its own sub-error
    #[error("Could not get WebGl2RenderingContext from canvas, because None was returned")]
    CanvasReturnedNoContext,

    // @todo: move this into its own sub-error
    #[error(
        "Could not create buffer because the attribute's location was not found in the program"
    )]
    AttributeLocationNotFoundCreateBufferError,
    #[error("Could not create buffer because no WebGL2RenderingContext was provided")]
    NoContextCreateBufferError,
    #[error("Could not create buffer because buffer link's associated program was not found from the program_id")]
    ProgramNotFoundCreateBufferError,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RendererBuilder<
    VertexShaderId: Id = DefaultId,
    FragmentShaderId: Id = DefaultId,
    ProgramId: Id = DefaultId,
    UniformId: Id + IdName = DefaultId,
    BufferId: Id + IdName = DefaultId,
    UserCtx = (),
> {
    canvas: Option<HtmlCanvasElement>,
    gl: Option<WebGl2RenderingContext>,
    vertex_shader_sources: HashMap<VertexShaderId, String>,
    fragment_shader_sources: HashMap<FragmentShaderId, String>,
    vertex_shaders: HashMap<VertexShaderId, WebGlShader>,
    fragment_shaders: HashMap<FragmentShaderId, WebGlShader>,
    program_links: HashSet<ProgramLink<ProgramId, VertexShaderId, FragmentShaderId>>,
    programs: HashMap<ProgramId, WebGlProgram>,
    uniform_links: HashSet<UniformLink<ProgramId, UniformId, UserCtx>>,
    uniforms: HashSet<Uniform<ProgramId, UniformId, UserCtx>>,
    buffer_links: HashSet<BufferLink<ProgramId, BufferId, UserCtx>>,
    buffers: HashSet<Buffer<ProgramId, BufferId, UserCtx>>,
    render_callback: Option<
        RenderCallback<VertexShaderId, FragmentShaderId, ProgramId, UniformId, BufferId, UserCtx>,
    >,
    user_ctx: Option<UserCtx>,
}

/// Public API
impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id + IdName,
        UserCtx,
    > RendererBuilder<VertexShaderId, FragmentShaderId, ProgramId, UniformId, BufferId, UserCtx>
{
    /// Save the canvas that will be rendered to and get its associated WebGL2 rendering context
    pub fn set_canvas(&mut self, canvas: HtmlCanvasElement) -> &mut Self {
        self.canvas = Some(canvas);

        self
    }

    /// Saves a fragment shader source and its corresponding id
    pub fn add_fragment_shader_src(
        &mut self,
        id: FragmentShaderId,
        fragment_shader_src: impl Into<String>,
    ) -> &mut Self {
        self.fragment_shader_sources
            .insert(id, fragment_shader_src.into());

        self
    }

    /// Saves a vertex shader source and its corresponding id
    pub fn add_vertex_shader_src(
        &mut self,
        id: VertexShaderId,
        vertex_shader_src: impl Into<String>,
    ) -> &mut Self {
        self.vertex_shader_sources
            .insert(id, vertex_shader_src.into());

        self
    }

    /// Saves a link between a vertex shader id and a fragment shader id.
    ///
    /// During the Renderer build process, this `program_link` is used to link a new WebGL2 program
    /// together by associating the vertex shader id and the fragment shader id with their corresponding compiled shaders.
    pub fn add_program_link(
        &mut self,
        program_link: impl Into<ProgramLink<ProgramId, VertexShaderId, FragmentShaderId>>,
    ) -> &mut Self {
        self.program_links.insert(program_link.into());

        self
    }

    /// Save a callback that will be called each time it is time to render a new frame
    pub fn set_render_callback(
        &mut self,
        render_callback: impl Into<
            RenderCallback<
                VertexShaderId,
                FragmentShaderId,
                ProgramId,
                UniformId,
                BufferId,
                UserCtx,
            >,
        >,
    ) -> &mut Self {
        self.render_callback = Some(render_callback.into());

        self
    }

    /// Save as arbitrary user context that can be accessed from within the render callback
    ///
    /// This can include stateful data and anything else that might be necessary to access
    /// while performing a render.
    pub fn set_user_ctx(&mut self, ctx: impl Into<UserCtx>) -> &mut Self {
        self.user_ctx = Some(ctx.into());

        self
    }

    /// Saves a link that will be used to build a uniform at build time.
    ///
    /// I.e. once all WebGL shaders are compiled and all programs are linked,
    /// all uniforms will be found within their associated programs, and will be
    /// saved with their associated update functions.
    pub fn add_uniform_link(
        &mut self,
        uniform_link: impl Into<UniformLink<ProgramId, UniformId, UserCtx>>,
    ) -> &mut Self {
        self.uniform_links.insert(uniform_link.into());

        self
    }

    /// Saves a link that will be used to build a buffer/attribute pair at build time.
    pub fn add_buffer_link(
        &mut self,
        buffer_link: impl Into<BufferLink<ProgramId, BufferId, UserCtx>>,
    ) -> &mut Self {
        self.buffer_links.insert(buffer_link.into());

        self
    }

    /// Compiles all vertex shaders and fragment shaders.
    /// Links together any programs that have been specified.
    /// Outputs the final Renderer.
    pub fn build(
        mut self,
    ) -> Result<
        Renderer<VertexShaderId, FragmentShaderId, ProgramId, UniformId, BufferId, UserCtx>,
        RendererBuilderError,
    > {
        self.save_webgl_context_from_canvas()?;
        self.compile_fragment_shaders()?;
        self.compile_vertex_shaders()?;
        self.link_programs()?;
        self.build_uniforms()?;
        self.create_buffers()?;

        let renderer = Renderer {
            canvas: self
                .canvas
                .ok_or(RendererBuilderError::NoCanvasBuildError)?,
            gl: self.gl.ok_or(RendererBuilderError::NoContextBuildError)?,
            fragment_shaders: self.fragment_shaders,
            vertex_shaders: self.vertex_shaders,
            programs: self.programs,
            render_callback: self
                .render_callback
                .ok_or(RendererBuilderError::NoRenderCallbackBuildError)?,
            user_ctx: self.user_ctx,
            uniforms: self.uniforms,
            buffers: self.buffers,
        };

        Ok(renderer)
    }
}

/// Private API
impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id + IdName,
        UserCtx,
    > RendererBuilder<VertexShaderId, FragmentShaderId, ProgramId, UniformId, BufferId, UserCtx>
{
    /// Gets the WebGL2 context from the canvas saved in state and saves the context in state
    fn save_webgl_context_from_canvas(&mut self) -> Result<&mut Self, RendererBuilderError> {
        let canvas = self
            .canvas
            .as_ref()
            .ok_or(RendererBuilderError::CanvasReturnedNoContext)?;
        let gl = Self::context_from_canvas(&canvas)?;
        self.gl = Some(gl);

        Ok(self)
    }

    /// Get the WebGL2 rendering context from a canvas
    fn context_from_canvas(
        canvas: &HtmlCanvasElement,
    ) -> Result<WebGl2RenderingContext, RendererBuilderError> {
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
        for program_link in self.program_links.iter() {
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

            let program_id = program_link.program_id().clone();

            self.programs.insert(program_id, program);
        }

        Ok(self)
    }

    /// Find the uniform's position in a shader and constructs necessary data for each uniform.
    fn build_uniform(
        &self,
        program_uniform_link: &UniformLink<ProgramId, UniformId, UserCtx>,
    ) -> Result<Uniform<ProgramId, UniformId, UserCtx>, RendererBuilderError> {
        let program_id = program_uniform_link.program_id().clone();
        let program = self
            .programs
            .get(&program_id)
            .ok_or(RendererBuilderError::ProgramNotFoundBuildUniformsError)?;

        let gl = self
            .gl
            .as_ref()
            .ok_or(RendererBuilderError::NoContextBuildError)?;

        let uniform_id = program_uniform_link.uniform_id().clone();
        let uniform_location = gl
            .get_uniform_location(program, &uniform_id.name())
            .ok_or(RendererBuilderError::UniformLocationNotFoundLinkBuildUniformError)?;
        let update_callback = program_uniform_link.update_callback();
        let uniform = Uniform::new(program_id, uniform_id, uniform_location, update_callback);

        Ok(uniform)
    }

    /// Creates a WebGL buffer for each BufferLink that was supplied using the create_callback.
    fn create_buffers(&mut self) -> Result<&mut Self, RendererBuilderError> {
        let gl = self
            .gl
            .as_ref()
            .ok_or(RendererBuilderError::NoContextCreateBufferError)?;
        let now = Date::now();
        let user_ctx = self.user_ctx.as_ref();

        for buffer_link in &self.buffer_links {
            let program_id = buffer_link.program_id().clone();
            let buffer_id = buffer_link.buffer_id().clone();

            let program = self
                .programs
                .get(&program_id)
                .ok_or(RendererBuilderError::ProgramNotFoundCreateBufferError)?;

            // webgl returns `-1` if the attribute location was not found
            let attribute_location: AttributeLocation =
                match gl.get_attrib_location(program, &buffer_id.name()) {
                    -1 => Err(RendererBuilderError::AttributeLocationNotFoundCreateBufferError)?,
                    attribute_location => attribute_location.into(),
                };

            let webgl_buffer = buffer_link.create_buffer(gl, now, &attribute_location, user_ctx);
            let update_callback = buffer_link.update_callback();
            let should_update_callback = buffer_link.should_update_callback();

            let buffer = Buffer::new(
                program_id,
                buffer_id,
                webgl_buffer,
                attribute_location,
                update_callback,
                should_update_callback,
            );

            self.buffers.insert(buffer);
        }

        Ok(self)
    }

    /// Finds all uniform's position in its corresponding program and builds a wrapper for it
    fn build_uniforms(&mut self) -> Result<&mut Self, RendererBuilderError> {
        for program_uniform_link in self.uniform_links.iter() {
            let uniform = self.build_uniform(program_uniform_link)?;
            self.uniforms.insert(uniform);
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

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id + IdName,
        UserCtx,
    > Default
    for RendererBuilder<VertexShaderId, FragmentShaderId, ProgramId, UniformId, BufferId, UserCtx>
{
    fn default() -> Self {
        Self {
            canvas: Default::default(),
            gl: Default::default(),
            vertex_shader_sources: Default::default(),
            fragment_shader_sources: Default::default(),
            vertex_shaders: Default::default(),
            fragment_shaders: Default::default(),
            program_links: Default::default(),
            programs: Default::default(),
            render_callback: Default::default(),
            user_ctx: Default::default(),
            uniform_links: Default::default(),
            uniforms: Default::default(),
            buffer_links: Default::default(),
            buffers: Default::default(),
        }
    }
}
