use super::animation_callback::AnimationCallback;
use super::animation_handle::AnimationHandle;
use super::attribute_location::AttributeLocation;
use super::attribute::Attribute;
use super::attribute_link::AttributeLink;
use super::default_id::DefaultId;
use super::framebuffer::Framebuffer;
use super::framebuffer_link::FramebufferLink;
use super::id::Id;
use super::id_name::IdName;
use super::render_callback::RenderCallback;
use super::texture::Texture;
use super::texture_link::TextureLink;
use super::uniform::Uniform;
use super::uniform_context::UniformContext;
use super::uniform_link::UniformLink;
use super::{program_link::ProgramLink, shader_type::ShaderType};
use std::fmt::Debug;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};
use thiserror::Error;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Renderer<
    VertexShaderId: Id = DefaultId,
    FragmentShaderId: Id = DefaultId,
    ProgramId: Id = DefaultId,
    UniformId: Id + IdName = DefaultId,
    BufferId: Id + IdName = DefaultId,
    TextureId: Id = DefaultId,
    FramebufferId: Id = DefaultId,
    UserCtx: 'static = (),
> {
    canvas: HtmlCanvasElement,
    gl: WebGl2RenderingContext,
    fragment_shaders: HashMap<FragmentShaderId, WebGlShader>,
    vertex_shaders: HashMap<VertexShaderId, WebGlShader>,
    programs: HashMap<ProgramId, WebGlProgram>,
    render_callback: RenderCallback<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        TextureId,
        FramebufferId,
        UserCtx,
    >,
    uniforms: HashMap<UniformId, Uniform<ProgramId, UniformId, UserCtx>>,
    user_ctx: Option<UserCtx>,
    buffers: HashMap<BufferId, Attribute<ProgramId, BufferId, UserCtx>>,
    textures: HashMap<TextureId, Texture<TextureId>>,
    framebuffers: HashMap<FramebufferId, Framebuffer<FramebufferId>>,
}

/// Public API
impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        UserCtx,
    >
    Renderer<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        TextureId,
        FramebufferId,
        UserCtx,
    >
{
    pub fn builder() -> RendererBuilder<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        TextureId,
        FramebufferId,
        UserCtx,
    > {
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

    pub fn uniforms(&self) -> &HashMap<UniformId, Uniform<ProgramId, UniformId, UserCtx>> {
        &self.uniforms
    }

    pub fn buffers(&self) -> &HashMap<BufferId, Attribute<ProgramId, BufferId, UserCtx>> {
        &self.buffers
    }

    pub fn textures(&self) -> &HashMap<TextureId, Texture<TextureId>> {
        &self.textures
    }

    pub fn framebuffers(&self) -> &HashMap<FramebufferId, Framebuffer<FramebufferId>> {
        &self.framebuffers
    }

    // @todo - enable ctx to be returned unconditionally (depending on if it's set or not)
    pub fn user_ctx(&self) -> Option<&UserCtx> {
        self.user_ctx.as_ref()
    }

    /// Updates a single uniform using the previously given update function. If no function was supplied,
    /// then this is a no-op.
    ///
    /// Calls "use_program" on the appropriate program before each uniform's update function (so this is not
    /// necessary to do within the callback itself, unless you need to change programs, for whatever reason).
    pub fn update_uniform(&self, uniform_id: &UniformId) -> &Self {
        let now = Self::now();
        let user_ctx = self.user_ctx();
        let gl = self.gl();
        let programs = self.programs();
        let uniform = self
            .uniforms
            .get(&uniform_id)
            .expect("UniformId should exist in registered uniforms");

        uniform.update(gl, now, user_ctx, programs);

        self
    }

    /// Iterates through all saved uniforms and updates them using their associated update callbacks.
    pub fn update_uniforms(&self) -> &Self {
        for (uniform_id, _) in &self.uniforms {
            self.update_uniform(uniform_id);
        }

        self
    }

    /// Updates a single buffer using the passed-in update callback.
    ///
    /// This function does NOT check whether the buffer SHOULD be updated before making the update call.
    ///
    /// Calls "use_program" on the appropriate program before each uniform's update function
    /// (so this is not necessary to do within the callback itself, unless you need to change programs, for
    /// whatever reason).
    ///
    /// Binds the correct buffer before the update callback is called, so this may be omitted.
    pub fn update_buffer(&self, buffer_id: &BufferId) -> &Self {
        let now = Self::now();
        let user_ctx = self.user_ctx();
        let gl = self.gl();

        let buffer = self
            .buffers
            .get(buffer_id)
            .expect("BufferId should exist in registered buffers");

        // bind the corresponding program
        let program_id = buffer.program_id();
        let program = self.programs().get(program_id);
        self.gl().use_program(program);

        // bind the corresponding buffer
        let webgl_buffer = buffer.webgl_buffer();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(webgl_buffer));
        buffer.update(self.gl(), now, user_ctx);
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

        self
    }

    /// Iterates through all saved buffers and updates them using their associated update callbacks.
    ///
    /// This function DOES check whether a buffer SHOULD be updated before calling its associated updated callback.
    ///
    /// If no should_update_callback was provided, then it is assumed that the buffer should be updated.
    pub fn update_buffers(&self) -> &Self {
        let now = Self::now();
        let user_ctx = self.user_ctx();
        let gl = self.gl();

        for (buffer_id, buffer) in &self.buffers {
            if !buffer.should_update(gl, now, user_ctx) {
                continue;
            }

            self.update_buffer(buffer_id);
        }

        self
    }

    pub fn render(&self) -> &Self {
        self.render_callback.call(self);

        self
    }

    /// Begins the animation process.
    ///
    /// If no animation callback has been provided, then the empty animation callback is run.
    pub fn into_animation_handle(
        self,
        animation_callback: AnimationCallback<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            TextureId,
            FramebufferId,
            UserCtx,
        >,
    ) -> AnimationHandle<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        TextureId,
        FramebufferId,
        UserCtx,
    > {
        AnimationHandle::new(animation_callback, self)
    }

    /// Gets current DOMHighResTimeStamp from performance.now()
    ///
    /// WebGL is limited to an f32, so using performance.now() (for now) to limit the size of the f64
    fn now() -> f64 {
        window().unwrap().performance().unwrap().now()
    }
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
    #[error("Could not build uniforms because no WebGL2RenderingContext was provided")]
    NoContextBuildUniformsError,
    #[error("Could not build uniforms because the associated program_id could no be found")]
    ProgramNotFoundBuildUniformsError,
    #[error(
        "Could not build uniforms because the uniform's location was not found in the program"
    )]
    UniformLocationNotFoundBuildUniformsError,

    // @todo: move this into its own sub-error
    #[error("Could not initialize uniforms because no WebGL2RenderingContext was provided")]
    NoContextInitializeUniformsError,

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

    #[error("Could not create texture because no WebGL2RenderingContext was provided")]
    NoContextCreateTextureError,

    #[error("Could not create framebuffer because no WebGL2RenderingContext was provided")]
    NoContextCreateFramebufferError,
}

#[derive(Debug, Clone)]
pub struct RendererBuilder<
    VertexShaderId: Id = DefaultId,
    FragmentShaderId: Id = DefaultId,
    ProgramId: Id = DefaultId,
    UniformId: Id + IdName = DefaultId,
    BufferId: Id + IdName = DefaultId,
    TextureId: Id = DefaultId,
    FramebufferId: Id = DefaultId,
    UserCtx: 'static = (),
> {
    canvas: Option<HtmlCanvasElement>,
    gl: Option<WebGl2RenderingContext>,
    vertex_shader_sources: HashMap<VertexShaderId, String>,
    fragment_shader_sources: HashMap<FragmentShaderId, String>,
    vertex_shaders: HashMap<VertexShaderId, WebGlShader>,
    fragment_shaders: HashMap<FragmentShaderId, WebGlShader>,
    program_links: HashSet<ProgramLink<ProgramId, VertexShaderId, FragmentShaderId, UserCtx>>,
    programs: HashMap<ProgramId, WebGlProgram>,
    uniform_links: HashSet<UniformLink<ProgramId, UniformId, UserCtx>>,
    uniforms: HashMap<UniformId, Uniform<ProgramId, UniformId, UserCtx>>,
    buffer_links: HashSet<AttributeLink<ProgramId, BufferId, UserCtx>>,
    buffers: HashMap<BufferId, Attribute<ProgramId, BufferId, UserCtx>>,
    texture_links: HashSet<TextureLink<TextureId, UserCtx>>,
    textures: HashMap<TextureId, Texture<TextureId>>,
    framebuffer_links: HashSet<
        FramebufferLink<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            TextureId,
            FramebufferId,
            UserCtx,
        >,
    >,
    framebuffers: HashMap<FramebufferId, Framebuffer<FramebufferId>>,
    render_callback: Option<
        RenderCallback<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            TextureId,
            FramebufferId,
            UserCtx,
        >,
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
        TextureId: Id,
        FramebufferId: Id,
        UserCtx: 'static,
    >
    RendererBuilder<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        TextureId,
        FramebufferId,
        UserCtx,
    >
{
    /// This is the only internal storage available publicly from the builder,
    /// because it is necessary to use it during the build process for framebuffers.
    pub fn texture(&self, texture_id: &TextureId) -> Option<&Texture<TextureId>> {
        self.textures.get(texture_id)
    }

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
        program_link: impl Into<ProgramLink<ProgramId, VertexShaderId, FragmentShaderId, UserCtx>>,
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
                TextureId,
                FramebufferId,
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
        attribute_link: impl Into<AttributeLink<ProgramId, BufferId, UserCtx>>,
    ) -> &mut Self {
        self.buffer_links.insert(attribute_link.into());

        self
    }

    /// Saves a link that will be used to build a buffer/attribute pair at build time.
    pub fn add_texture_link(
        &mut self,
        texture_link: impl Into<TextureLink<TextureId, UserCtx>>,
    ) -> &mut Self {
        self.texture_links.insert(texture_link.into());

        self
    }

    /// Saves a link that will be used to build a buffer/attribute pair at build time.
    pub fn add_framebuffer_link(
        &mut self,
        framebuffer_link: impl Into<
            FramebufferLink<
                VertexShaderId,
                FragmentShaderId,
                ProgramId,
                UniformId,
                BufferId,
                TextureId,
                FramebufferId,
                UserCtx,
            >,
        >,
    ) -> &mut Self {
        self.framebuffer_links.insert(framebuffer_link.into());

        self
    }

    /// Compiles all vertex shaders and fragment shaders.
    /// Links together any programs that have been specified.
    /// Outputs the final Renderer.
    pub fn build(
        mut self,
    ) -> Result<
        Renderer<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            TextureId,
            FramebufferId,
            UserCtx,
        >,
        RendererBuilderError,
    > {
        // the order here is fairly important
        self.save_webgl_context_from_canvas()?;
        self.compile_fragment_shaders()?;
        self.compile_vertex_shaders()?;
        self.link_programs()?;
        self.build_uniforms()?;
        self.create_buffers()?;
        self.create_textures()?;
        self.create_framebuffers()?;

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
            textures: self.textures,
            framebuffers: self.framebuffers,
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
        TextureId: Id,
        FramebufferId: Id,
        UserCtx,
    >
    RendererBuilder<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        TextureId,
        FramebufferId,
        UserCtx,
    >
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
        uniform_link: &UniformLink<ProgramId, UniformId, UserCtx>,
    ) -> Result<Uniform<ProgramId, UniformId, UserCtx>, RendererBuilderError> {
        let uniform_id = uniform_link.uniform_id().clone();
        let program_ids = uniform_link.program_ids().clone();
        let gl = self
            .gl
            .as_ref()
            .ok_or(RendererBuilderError::NoContextBuildUniformsError)?;
        let now = Self::now();
        let user_ctx = self.user_ctx.as_ref();
        let initialize_callback = uniform_link.initialize_callback();
        let should_update_callback = uniform_link.should_update_callback();
        let update_callback = uniform_link.update_callback();
        let mut uniform_locations = HashMap::new();

        for program_id in &program_ids {
            let program = self
                .programs
                .get(&program_id)
                .ok_or(RendererBuilderError::ProgramNotFoundBuildUniformsError)?;

            gl.use_program(Some(program));

            let uniform_location = gl
                .get_uniform_location(program, &uniform_id.name())
                .ok_or(RendererBuilderError::UniformLocationNotFoundBuildUniformsError)?;
            let uniform_context = UniformContext::new(gl, now, &uniform_location, user_ctx);
            (initialize_callback)(&uniform_context);
            uniform_locations.insert(program_id.to_owned(), uniform_location);

            gl.use_program(None);
        }

        let uniform = Uniform::new(
            program_ids,
            uniform_id,
            uniform_locations,
            initialize_callback,
            update_callback,
            should_update_callback,
        );

        Ok(uniform)
    }

    /// Creates a WebGL buffer for each AttributeLink that was supplied using the create_callback.
    fn create_buffers(&mut self) -> Result<&mut Self, RendererBuilderError> {
        let gl = self
            .gl
            .as_ref()
            .ok_or(RendererBuilderError::NoContextCreateBufferError)?;
        let now = Self::now();
        let user_ctx = self.user_ctx.as_ref();

        for attribute_link in &self.buffer_links {
            let program_id = attribute_link.program_id().clone();
            let buffer_id = attribute_link.buffer_id().clone();

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

            let webgl_buffer = attribute_link.create_buffer(gl, now, &attribute_location, user_ctx);
            gl.enable_vertex_attrib_array(attribute_location.into());

            let update_callback = attribute_link.update_callback();
            let should_update_callback = attribute_link.should_update_callback();

            let buffer = Attribute::new(
                program_id,
                buffer_id.clone(),
                webgl_buffer,
                attribute_location,
                update_callback,
                should_update_callback,
            );

            self.buffers.insert(buffer_id, buffer);
        }

        Ok(self)
    }

    /// Creates a WebGL texture for each Texture that was supplied using the create_texture callback
    fn create_textures(&mut self) -> Result<&mut Self, RendererBuilderError> {
        let gl = self
            .gl
            .as_ref()
            .ok_or(RendererBuilderError::NoContextCreateTextureError)?;
        let now = Self::now();
        let user_ctx = self.user_ctx.as_ref();

        for texture_link in &self.texture_links {
            let texture_id = texture_link.texture_id().clone();
            let webgl_texture = texture_link.create_texture(gl, now, user_ctx);
            let texture = Texture::new(texture_id.clone(), webgl_texture);

            self.textures.insert(texture_id, texture);
        }

        Ok(self)
    }

    /// Creates a WebGL Framebuffer for each FramebufferLink that was supplied using the callback
    fn create_framebuffers(&mut self) -> Result<&mut Self, RendererBuilderError> {
        let gl = self
            .gl
            .as_ref()
            .ok_or(RendererBuilderError::NoContextCreateFramebufferError)?;
        let now = Self::now();
        let user_ctx = self.user_ctx.as_ref();

        for framebuffer_link in &self.framebuffer_links {
            let framebuffer_id = framebuffer_link.framebuffer_id().clone();
            let webgl_framebuffer = framebuffer_link.create_framebuffer(gl, now, self, user_ctx);
            let framebuffer = Framebuffer::new(framebuffer_id.clone(), webgl_framebuffer);

            self.framebuffers.insert(framebuffer_id, framebuffer);
        }

        Ok(self)
    }

    /// Finds all uniform's position in its corresponding program and builds a wrapper for it
    fn build_uniforms(&mut self) -> Result<&mut Self, RendererBuilderError> {
        for uniform_link in self.uniform_links.iter() {
            let uniform_id = uniform_link.uniform_id().clone();
            let uniform = self.build_uniform(uniform_link)?;
            self.uniforms.insert(uniform_id, uniform);
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

    /// Gets current DOMHighResTimeStamp from performance.now()
    ///
    /// WebGL is limited to an f32, so using performance.now() (for now) to limit the size of the f64
    fn now() -> f64 {
        window().unwrap().performance().unwrap().now()
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
        TextureId: Id,
        FramebufferId: Id,
        UserCtx,
    > Default
    for RendererBuilder<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        TextureId,
        FramebufferId,
        UserCtx,
    >
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
            texture_links: Default::default(),
            textures: Default::default(),
            framebuffer_links: Default::default(),
            framebuffers: Default::default(),
        }
    }
}
