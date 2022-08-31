use crate::{
    Attribute, AttributeLink, Bridge, Buffer, BufferLink, BuildRendererError, CompileShaderError,
    CreateAttributeError, CreateBufferError, CreateTextureError, CreateTransformFeedbackError,
    CreateUniformError, CreateVAOError, Framebuffer, FramebufferLink, GetContextCallback, Id,
    IdDefault, IdName, IntoJsWrapper, LinkProgramError, ProgramLink, RenderCallback,
    RendererBuilderError, RendererHandle, RendererJs, RendererJsInner, SaveContextError,
    ShaderType, Texture, TextureLink, TransformFeedbackLink, Uniform, UniformContext, UniformLink,
    WebGlContextError,
};
use log::error;
use std::{
    any::Any,
    collections::{HashMap, HashSet},
};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    window, HtmlAnchorElement, HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram,
    WebGlShader, WebGlTransformFeedback, WebGlVertexArrayObject,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Renderer<
    VertexShaderId: Id = IdDefault,
    FragmentShaderId: Id = IdDefault,
    ProgramId: Id = IdDefault,
    UniformId: Id + IdName = IdDefault,
    BufferId: Id = IdDefault,
    AttributeId: Id + IdName = IdDefault,
    TextureId: Id = IdDefault,
    FramebufferId: Id = IdDefault,
    TransformFeedbackId: Id = IdDefault,
    VertexArrayObjectId: Id = IdDefault,
    UserCtx: Clone + 'static = (),
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
        AttributeId,
        TextureId,
        FramebufferId,
        TransformFeedbackId,
        VertexArrayObjectId,
        UserCtx,
    >,
    uniforms: HashMap<UniformId, Uniform<ProgramId, UniformId>>,
    user_ctx: Option<UserCtx>,
    attributes: HashMap<AttributeId, Attribute<VertexArrayObjectId, BufferId, AttributeId>>,
    buffers: HashMap<BufferId, Buffer<BufferId>>,
    textures: HashMap<TextureId, Texture<TextureId>>,
    vertex_array_objects: HashMap<VertexArrayObjectId, WebGlVertexArrayObject>,
    framebuffers: HashMap<FramebufferId, Framebuffer<FramebufferId>>,
    transform_feedbacks: HashMap<TransformFeedbackId, WebGlTransformFeedback>,
}

/// Public API
impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id,
        AttributeId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        TransformFeedbackId: Id,
        VertexArrayObjectId: Id,
        UserCtx: Clone,
    >
    Renderer<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        TransformFeedbackId,
        VertexArrayObjectId,
        UserCtx,
    >
{
    pub fn builder() -> RendererBuilder<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        TransformFeedbackId,
        VertexArrayObjectId,
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

    pub fn fragment_shader(&self, fragment_shader_id: &FragmentShaderId) -> Option<&WebGlShader> {
        self.fragment_shaders.get(fragment_shader_id)
    }

    pub fn fragment_shaders(&self) -> &HashMap<FragmentShaderId, WebGlShader> {
        &self.fragment_shaders
    }

    pub fn vertex_shader(&self, vertex_shader_id: &VertexShaderId) -> Option<&WebGlShader> {
        self.vertex_shaders.get(vertex_shader_id)
    }

    pub fn vertex_shaders(&self) -> &HashMap<VertexShaderId, WebGlShader> {
        &self.vertex_shaders
    }

    pub fn program(&self, program_id: &ProgramId) -> Option<&WebGlProgram> {
        self.programs.get(program_id)
    }

    pub fn programs(&self) -> &HashMap<ProgramId, WebGlProgram> {
        &self.programs
    }

    pub fn uniform(&self, uniform_id: &UniformId) -> Option<&Uniform<ProgramId, UniformId>> {
        self.uniforms.get(uniform_id)
    }

    pub fn uniforms(&self) -> &HashMap<UniformId, Uniform<ProgramId, UniformId>> {
        &self.uniforms
    }

    pub fn buffer(&self, buffer_id: &BufferId) -> Option<&Buffer<BufferId>> {
        self.buffers.get(buffer_id)
    }

    pub fn buffers(&self) -> &HashMap<BufferId, Buffer<BufferId>> {
        &self.buffers
    }

    pub fn attribute(
        &self,
        attribute_id: &AttributeId,
    ) -> Option<&Attribute<VertexArrayObjectId, BufferId, AttributeId>> {
        self.attributes.get(attribute_id)
    }

    pub fn attributes(
        &self,
    ) -> &HashMap<AttributeId, Attribute<VertexArrayObjectId, BufferId, AttributeId>> {
        &self.attributes
    }

    pub fn texture(&self, texture_id: &TextureId) -> Option<&Texture<TextureId>> {
        self.textures.get(texture_id)
    }

    pub fn textures(&self) -> &HashMap<TextureId, Texture<TextureId>> {
        &self.textures
    }

    pub fn textures_by_id(
        &self,
        texture_ids: impl Into<Bridge<TextureId>>,
    ) -> Vec<&Texture<TextureId>> {
        let texture_ids: Bridge<_> = texture_ids.into();
        let texture_ids: Vec<_> = texture_ids.into();
        let mut textures = Vec::with_capacity(texture_ids.len());
        for texture_id in texture_ids {
            let texture = self.texture(&texture_id);
            if let Some(texture) = texture {
                textures.push(texture);
            }
        }
        textures
    }

    pub fn framebuffer(
        &self,
        framebuffer_id: &FramebufferId,
    ) -> Option<&Framebuffer<FramebufferId>> {
        self.framebuffers.get(framebuffer_id)
    }

    pub fn transform_feedback(
        &self,
        transform_feedback_id: &TransformFeedbackId,
    ) -> Option<&WebGlTransformFeedback> {
        self.transform_feedbacks.get(transform_feedback_id)
    }

    pub fn vao(&self, vao_id: &VertexArrayObjectId) -> Option<&WebGlVertexArrayObject> {
        self.vertex_array_objects.get(vao_id)
    }

    // @todo - enable ctx to be returned unconditionally (depending on if it's set or not)
    pub fn user_ctx(&self) -> Option<&UserCtx> {
        self.user_ctx.as_ref()
    }

    /// Switches to using new program and its associated VAO
    pub fn use_program(&self, program_id: &ProgramId) -> &Self {
        let program = self
            .programs
            .get(program_id)
            .expect("Program should exist for ProgramId");

        self.gl().use_program(Some(program));

        self
    }

    pub fn use_vao(&self, vao_id: &VertexArrayObjectId) -> &Self {
        let vao = self
            .vertex_array_objects
            .get(vao_id)
            .expect("VAO should exist for ProgramId");

        self.gl().bind_vertex_array(Some(vao));

        self
    }

    /// Updates a single uniform using the previously given update function. If no function was supplied,
    /// then this is a no-op.
    ///
    /// Calls "use_program" on the appropriate program before each uniform's update function (so this is not
    /// necessary to do within the callback itself, unless you need to change programs, for whatever reason).
    pub fn update_uniform(&self, uniform_id: &UniformId) -> &Self {
        let now = Self::now();
        let _user_ctx = self.user_ctx();
        let gl = self.gl();
        let programs = &self.programs;
        let uniform = self
            .uniforms
            .get(uniform_id)
            .expect("UniformId should exist in registered uniforms");

        uniform.update(gl, now, programs);

        self
    }

    /// Iterates through all saved uniforms and updates them using their associated update callbacks.
    pub fn update_uniforms(&self) -> &Self {
        for uniform_id in self.uniforms.keys() {
            self.update_uniform(uniform_id);
        }

        self
    }

    pub fn render(&self) -> &Self {
        // If `Renderer` has been instantiated through JavaScript, we have to treat this as a special case.
        // In order to convert `Renderer` into a `JsValue`, it must meet specific requirements for its generic arguments.
        // We can check this at runtime using `Any` and downcasting
        let rendered_as_js = (|| {
            if let Some(renderer) = (self as &dyn Any).downcast_ref::<RendererJsInner>() {
                if let Some(js_callback) = self.render_callback.b().as_ref() {
                    if let Err(err) = js_callback
                        // must clone the Renderer here, which could be potentially expensive
                        // @todo: find a way to not have to clone the entire Renderer struct
                        .call1(&JsValue::NULL, &renderer.clone().into_js_wrapper().into())
                    {
                        error!(
                            "Error occurred while calling JavaScript `render` callback: {err:?}"
                        );
                    }
                    return true;
                }
            }
            false
        })();

        // If not already renderer as a JavaScript callback, should now be rendered
        // as a Rust callback (or as a JavaScript called, without the `Renderer` argument supplied)
        if !rendered_as_js {
            self.render_callback.call(&self);
        }

        self
    }

    pub fn save_image(&self) {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        let a: HtmlAnchorElement = document.create_element("a").unwrap().dyn_into().unwrap();
        let data_url = self
            .canvas
            .to_data_url()
            .unwrap()
            .replace("image/png", "image/octet-stream");

        a.style().set_css_text("display: none;");
        a.set_href(&data_url);
        a.set_download("image.png");

        body.append_child(&a).unwrap();
        a.click();
        body.remove_child(&a).unwrap();
    }

    /// Begins the animation process.
    ///
    /// If no animation callback has been provided, then the empty animation callback is run.
    pub fn into_renderer_handle(
        self,
    ) -> RendererHandle<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        TransformFeedbackId,
        VertexArrayObjectId,
        UserCtx,
    > {
        self.into()
    }

    /// Gets current DOMHighResTimeStamp from performance.now()
    ///
    /// WebGL is limited to an f32, so using performance.now() (for now) to limit the size of the f64
    fn now() -> f64 {
        window().unwrap().performance().unwrap().now()
    }
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id,
        AttributeId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        TransformFeedbackId: Id,
        VertexArrayObjectId: Id,
        UserCtx: Clone,
    > AsRef<HtmlCanvasElement>
    for Renderer<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        TransformFeedbackId,
        VertexArrayObjectId,
        UserCtx,
    >
{
    fn as_ref(&self) -> &HtmlCanvasElement {
        self.canvas()
    }
}

impl From<RendererJsInner> for JsValue {
    fn from(renderer: RendererJsInner) -> Self {
        let js_renderer: RendererJs = renderer.into();
        js_renderer.into()
    }
}

#[derive(Debug, Clone)]
pub struct RendererBuilder<
    VertexShaderId: Id = IdDefault,
    FragmentShaderId: Id = IdDefault,
    ProgramId: Id = IdDefault,
    UniformId: Id + IdName = IdDefault,
    BufferId: Id = IdDefault,
    AttributeId: Id + IdName = IdDefault,
    TextureId: Id = IdDefault,
    FramebufferId: Id = IdDefault,
    TransformFeedbackId: Id = IdDefault,
    VertexArrayObjectId: Id = IdDefault,
    UserCtx: Clone + 'static = (),
> {
    canvas: Option<HtmlCanvasElement>,
    gl: Option<WebGl2RenderingContext>,
    vertex_shader_sources: HashMap<VertexShaderId, String>,
    fragment_shader_sources: HashMap<FragmentShaderId, String>,
    vertex_shaders: HashMap<VertexShaderId, WebGlShader>,
    fragment_shaders: HashMap<FragmentShaderId, WebGlShader>,
    program_links: HashSet<ProgramLink<ProgramId, VertexShaderId, FragmentShaderId>>,
    programs: HashMap<ProgramId, WebGlProgram>,
    uniform_links: HashSet<UniformLink<ProgramId, UniformId>>,
    uniforms: HashMap<UniformId, Uniform<ProgramId, UniformId>>,
    buffer_links: HashSet<BufferLink<BufferId>>,
    buffers: HashMap<BufferId, Buffer<BufferId>>,
    attribute_links: HashSet<AttributeLink<VertexArrayObjectId, BufferId, AttributeId>>,
    attribute_locations: HashMap<AttributeId, u32>,
    attributes: HashMap<AttributeId, Attribute<VertexArrayObjectId, BufferId, AttributeId>>,
    texture_links: HashSet<TextureLink<TextureId>>,
    textures: HashMap<TextureId, Texture<TextureId>>,
    framebuffer_links: HashSet<FramebufferLink<FramebufferId, TextureId>>,
    framebuffers: HashMap<FramebufferId, Framebuffer<FramebufferId>>,
    render_callback: Option<
        RenderCallback<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            AttributeId,
            TextureId,
            FramebufferId,
            TransformFeedbackId,
            VertexArrayObjectId,
            UserCtx,
        >,
    >,
    user_ctx: Option<UserCtx>,
    vertex_array_object_links: HashSet<VertexArrayObjectId>,
    vertex_array_objects: HashMap<VertexArrayObjectId, WebGlVertexArrayObject>,
    transform_feedback_links: HashSet<TransformFeedbackLink<TransformFeedbackId>>,
    transform_feedbacks: HashMap<TransformFeedbackId, WebGlTransformFeedback>,
    get_context_callback: GetContextCallback,
}

/// Public API
impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id,
        AttributeId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        TransformFeedbackId: Id,
        VertexArrayObjectId: Id,
        UserCtx: Clone + 'static,
    >
    RendererBuilder<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        TransformFeedbackId,
        VertexArrayObjectId,
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
        program_link: impl Into<ProgramLink<ProgramId, VertexShaderId, FragmentShaderId>>,
    ) -> &mut Self {
        let program_link = program_link.into();
        self.program_links.insert(program_link);

        self
    }

    pub fn add_program_links(
        &mut self,
        program_links: impl Into<Bridge<ProgramLink<ProgramId, VertexShaderId, FragmentShaderId>>>,
    ) -> &mut Self {
        let program_link_bridge: Bridge<ProgramLink<ProgramId, VertexShaderId, FragmentShaderId>> =
            program_links.into();
        let program_links: Vec<_> = program_link_bridge.into();

        for program_link in program_links {
            self.add_program_link(program_link);
        }

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
                AttributeId,
                TextureId,
                FramebufferId,
                TransformFeedbackId,
                VertexArrayObjectId,
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
        uniform_link: impl Into<UniformLink<ProgramId, UniformId>>,
    ) -> &mut Self {
        self.uniform_links.insert(uniform_link.into());

        self
    }

    pub fn add_uniform_links(
        &mut self,
        uniform_links: impl Into<Bridge<UniformLink<ProgramId, UniformId>>>,
    ) -> &mut Self {
        let uniform_link_bridge: Bridge<_> = uniform_links.into();
        let uniform_links: Vec<_> = uniform_link_bridge.into();

        for uniform_link in uniform_links {
            self.add_uniform_link(uniform_link);
        }

        self
    }

    /// Saves a link that will be used to build a WebGL buffer at build time.
    pub fn add_buffer_link(&mut self, buffer_link: impl Into<BufferLink<BufferId>>) -> &mut Self {
        self.buffer_links.insert(buffer_link.into());

        self
    }

    pub fn add_buffer_links(
        &mut self,
        buffer_links: impl Into<Bridge<BufferLink<BufferId>>>,
    ) -> &mut Self {
        let buffer_link_bridge: Bridge<_> = buffer_links.into();
        let buffer_links: Vec<_> = buffer_link_bridge.into();

        for buffer_link in buffer_links {
            self.add_buffer_link(buffer_link);
        }

        self
    }

    /// Saves a link that will be used to build a a WebGL attribute at build time.
    pub fn add_attribute_link(
        &mut self,
        attribute_link: impl Into<AttributeLink<VertexArrayObjectId, BufferId, AttributeId>>,
    ) -> &mut Self {
        let attribute_link = attribute_link.into();
        let attribute_id = attribute_link.attribute_id().to_owned();
        let new_attribute_location = self.attribute_links.len() as u32;
        self.attribute_links.insert(attribute_link);
        self.attribute_locations
            .insert(attribute_id, new_attribute_location);

        self
    }

    pub fn add_attribute_links(
        &mut self,
        attribute_links: impl Into<Bridge<AttributeLink<VertexArrayObjectId, BufferId, AttributeId>>>,
    ) -> &mut Self {
        let attribute_link_bridge: Bridge<_> = attribute_links.into();
        let attribute_links: Vec<_> = attribute_link_bridge.into();

        for attribute_link in attribute_links {
            self.add_attribute_link(attribute_link);
        }

        self
    }

    /// Saves a link that will be used to build a buffer/attribute pair at build time.
    pub fn add_texture_link(
        &mut self,
        texture_link: impl Into<TextureLink<TextureId>>,
    ) -> &mut Self {
        self.texture_links.insert(texture_link.into());

        self
    }

    pub fn add_texture_links(
        &mut self,
        texture_links: impl Into<Bridge<TextureLink<TextureId>>>,
    ) -> &mut Self {
        let texture_link_bridge: Bridge<_> = texture_links.into();
        let texture_links: Vec<_> = texture_link_bridge.into();

        for texture_link in texture_links {
            self.add_texture_link(texture_link);
        }

        self
    }

    /// Saves a link that will be used to build a framebuffer at build time
    pub fn add_framebuffer_link(
        &mut self,
        framebuffer_link: impl Into<FramebufferLink<FramebufferId, TextureId>>,
    ) -> &mut Self {
        self.framebuffer_links.insert(framebuffer_link.into());

        self
    }

    pub fn add_framebuffer_links(
        &mut self,
        framebuffer_links: impl Into<Bridge<FramebufferLink<FramebufferId, TextureId>>>,
    ) -> &mut Self {
        let framebuffer_link_bridge: Bridge<_> = framebuffer_links.into();
        let framebuffer_links: Vec<_> = framebuffer_link_bridge.into();

        for framebuffer_link in framebuffer_links {
            self.add_framebuffer_link(framebuffer_link);
        }

        self
    }

    /// Saves a link that will be used to build a transformFeedback at build time
    pub fn add_transform_feedback_link(
        &mut self,
        transform_feedback_link: impl Into<TransformFeedbackLink<TransformFeedbackId>>,
    ) -> &mut Self {
        self.transform_feedback_links
            .insert(transform_feedback_link.into());

        self
    }

    pub fn add_transform_feedback_links(
        &mut self,
        transform_feedback_links: impl Into<Bridge<TransformFeedbackLink<TransformFeedbackId>>>,
    ) -> &mut Self {
        let transform_feedback_link_bridge: Bridge<_> = transform_feedback_links.into();
        let transform_feedback_links: Vec<_> = transform_feedback_link_bridge.into();

        for transform_feedback_link in transform_feedback_links {
            self.add_transform_feedback_link(transform_feedback_link);
        }

        self
    }

    /// Saves an id that will be used to create a VAO at build time
    ///
    /// This VAO can then be referenced by `AttributeLink`s
    pub fn add_vao_link(
        &mut self,
        vertex_array_object_id: impl Into<VertexArrayObjectId>,
    ) -> &mut Self {
        self.vertex_array_object_links
            .insert(vertex_array_object_id.into());

        self
    }

    pub fn add_vao_links(
        &mut self,
        vao_links: impl Into<Bridge<VertexArrayObjectId>>,
    ) -> &mut Self {
        let vao_link_bridge: Bridge<_> = vao_links.into();
        let vao_links: Vec<_> = vao_link_bridge.into();
        let vao_links: Vec<VertexArrayObjectId> = vao_links.into_iter().collect();

        for vao_link in vao_links {
            self.add_vao_link(vao_link);
        }

        self
    }

    pub fn set_get_context_callback(
        &mut self,
        get_context_callback: impl Into<GetContextCallback>,
    ) -> &mut Self {
        self.get_context_callback = get_context_callback.into();
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
            AttributeId,
            TextureId,
            FramebufferId,
            TransformFeedbackId,
            VertexArrayObjectId,
            UserCtx,
        >,
        RendererBuilderError,
    > {
        // the order here is fairly important
        self.save_webgl_context_from_canvas()?;
        self.compile_fragment_shaders()?;
        self.compile_vertex_shaders()?;
        self.create_vaos()?;
        self.link_programs()?;
        self.create_buffers()?;
        self.create_attributes()?;
        self.create_uniforms()?;
        self.create_textures()?;
        self.create_framebuffers()?;
        self.create_transform_feedbacks()?;

        let renderer = Renderer {
            canvas: self.canvas.ok_or(BuildRendererError::NoCanvas)?,
            gl: self.gl.ok_or(BuildRendererError::NoContext)?,
            fragment_shaders: self.fragment_shaders,
            vertex_shaders: self.vertex_shaders,
            programs: self.programs,
            render_callback: self
                .render_callback
                .ok_or(BuildRendererError::NoRenderCallback)?,
            user_ctx: self.user_ctx,
            uniforms: self.uniforms,
            buffers: self.buffers,
            textures: self.textures,
            framebuffers: self.framebuffers,
            attributes: self.attributes,
            vertex_array_objects: self.vertex_array_objects,
            transform_feedbacks: self.transform_feedbacks,
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
        BufferId: Id,
        AttributeId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        TransformFeedbackId: Id,
        VertexArrayObjectId: Id,
        UserCtx: Clone,
    >
    RendererBuilder<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        TransformFeedbackId,
        VertexArrayObjectId,
        UserCtx,
    >
{
    /// Gets the WebGL2 context from the canvas saved in state and saves the context in state
    fn save_webgl_context_from_canvas(&mut self) -> Result<&mut Self, RendererBuilderError> {
        let canvas = self
            .canvas
            .as_ref()
            .ok_or(SaveContextError::CanvasReturnedNoContext)?
            .to_owned();
        let gl = self.context_from_canvas(canvas)?;
        self.gl = Some(gl);

        Ok(self)
    }

    /// Get the WebGL2 rendering context from a canvas
    fn context_from_canvas(
        &self,
        canvas: HtmlCanvasElement,
    ) -> Result<WebGl2RenderingContext, WebGlContextError> {
        let gl = match &*self.get_context_callback {
            crate::Either::A(rust_callback) => (rust_callback)(canvas)?,
            crate::Either::B(js_callback) => {
                let result = js_callback.call1(&JsValue::NULL, canvas.as_ref());
                result.expect("Received error when trying call JavaScript `get_context_callback`")
                    .dyn_into()
                    .expect("Did not receive expected type `HtmlCanvasElement` from JavaScript function `get_context_callback`")
            }
        };
        Ok(gl)
    }

    /// Takes the list of fragment shader sources and their ids and saves compiled `WebGlShader`s to state
    fn compile_fragment_shaders(&mut self) -> Result<&mut Self, CompileShaderError> {
        for (id, fragment_shader_src) in self.fragment_shader_sources.iter() {
            let fragment_shader =
                self.compile_shader(id.clone(), ShaderType::FragmentShader, fragment_shader_src)?;
            self.fragment_shaders.insert((*id).clone(), fragment_shader);
        }

        Ok(self)
    }

    /// Takes the list of vertex shader sources and their ids and saves compiled `WebGlShader`s to state
    fn compile_vertex_shaders(&mut self) -> Result<&mut Self, CompileShaderError> {
        for (id, vertex_shader_src) in self.vertex_shader_sources.iter() {
            let vertex_shader =
                self.compile_shader(id.clone(), ShaderType::VertexShader, vertex_shader_src)?;
            self.vertex_shaders.insert((*id).clone(), vertex_shader);
        }

        Ok(self)
    }

    fn create_transform_feedbacks(&mut self) -> Result<&mut Self, CreateTransformFeedbackError> {
        let gl = self
            .gl
            .as_ref()
            .ok_or(CreateTransformFeedbackError::NoContext)?;

        for transform_feedback_link in self.transform_feedback_links.iter() {
            let transform_feedback_id = transform_feedback_link.transform_feedback_id().clone();
            let webgl_transform_feedback = gl
                .create_transform_feedback()
                .ok_or(CreateTransformFeedbackError::NoneWasReturned)?;
            self.transform_feedbacks
                .insert(transform_feedback_id, webgl_transform_feedback);
        }

        Ok(self)
    }

    /// Links together all of the vertex & fragment shaders that have been saved
    /// according to any ProgramLinks that were provided.
    ///
    /// If a ProgramLink does not correspond to an actual shader, returns an Error.
    fn link_programs(&mut self) -> Result<&mut Self, LinkProgramError> {
        for program_link in self.program_links.iter() {
            let program = self.link_program(program_link)?;
            let program_id = program_link.program_id();
            self.programs.insert(program_id.clone(), program);
        }

        Ok(self)
    }

    /// Find the uniform's position in a shader and constructs necessary data for each uniform.
    fn create_uniform(
        &self,
        uniform_link: &UniformLink<ProgramId, UniformId>,
    ) -> Result<Uniform<ProgramId, UniformId>, CreateUniformError> {
        let uniform_id = uniform_link.uniform_id().clone();
        let program_ids = uniform_link.program_ids().clone();
        let use_init_callback_for_update = uniform_link.use_init_callback_for_update();
        let gl = self.gl.as_ref().ok_or(CreateUniformError::NoContext)?;
        let now = Self::now();
        let _user_ctx = self.user_ctx.as_ref().map(Clone::clone);
        let initialize_callback = uniform_link.initialize_callback();
        let should_update_callback = uniform_link.should_update_callback();
        let update_callback = uniform_link.update_callback();
        let mut uniform_locations = HashMap::new();

        for program_id in &program_ids {
            let program = self
                .programs
                .get(program_id)
                .ok_or(CreateUniformError::ProgramNotFound)?;

            gl.use_program(Some(program));

            let uniform_location = gl.get_uniform_location(program, &uniform_id.name()).ok_or(
                CreateUniformError::UniformLocationNotFound {
                    uniform_id: uniform_id.name(),
                },
            )?;
            let uniform_context = UniformContext::new(gl.clone(), now, uniform_location.clone());
            initialize_callback.call_with_arg_into_js_value(&uniform_context);
            uniform_locations.insert(program_id.to_owned(), uniform_location.clone());

            gl.use_program(None);
        }

        let uniform = Uniform::new(
            program_ids,
            uniform_id,
            uniform_locations,
            initialize_callback,
            update_callback,
            should_update_callback,
            use_init_callback_for_update,
        );

        Ok(uniform)
    }

    /// Creates all WebGL buffers, using the passed in BufferLinks
    fn create_buffers(&mut self) -> Result<&mut Self, CreateBufferError> {
        let gl = self.gl.as_ref().ok_or(CreateBufferError::NoContext)?;
        let now = Self::now();

        for buffer_link in &self.buffer_links {
            let buffer_id = buffer_link.buffer_id().clone();
            let webgl_buffer = buffer_link.create_buffer(gl.clone(), now);
            let buffer = Buffer::new(buffer_id.clone(), webgl_buffer);
            self.buffers.insert(buffer_id, buffer);
        }

        Ok(self)
    }

    fn create_vaos(&mut self) -> Result<&mut Self, CreateVAOError> {
        let gl = self.gl.as_ref().ok_or(CreateVAOError::NoContext)?;

        for vao_id in self.vertex_array_object_links.iter() {
            let vao = gl
                .create_vertex_array()
                .ok_or(CreateVAOError::NoneWasReturned)?;
            self.vertex_array_objects.insert(vao_id.to_owned(), vao);
        }

        Ok(self)
    }

    /// Creates a WebGL attribute for each AttributeLink that was supplied using the create_callback
    fn create_attributes(&mut self) -> Result<&mut Self, CreateAttributeError> {
        let gl = self.gl.as_ref().ok_or(CreateAttributeError::NoContext)?;
        let now = Self::now();
        let _user_ctx = self.user_ctx.clone();

        for attribute_link in &self.attribute_links {
            let vao_ids = attribute_link.vao_ids();
            let buffer_id = attribute_link.buffer_id().clone();
            let attribute_id = attribute_link.attribute_id().clone();
            let webgl_buffer = self
                .buffers
                .get(&buffer_id)
                .ok_or(CreateAttributeError::BufferNotFound)?
                .webgl_buffer()
                .clone();
            let attribute_location = self
                .attribute_locations
                .get(&attribute_id)
                .ok_or(CreateAttributeError::AttributeLocationNotFound)?;

            if vao_ids.is_empty() {
                // initialize attribute on the default VAO context
                gl.bind_vertex_array(None);
                gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&webgl_buffer));
                gl.enable_vertex_attrib_array(*attribute_location);
                // create callback is expected to initialize its associated attribute
                // with a call to vertexAttribPointer,
                // which is saved in the associated VAO
                attribute_link.create_attribute(
                    gl.clone(),
                    now,
                    webgl_buffer.clone(),
                    attribute_location.into(),
                );
                gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
            } else {
                // initialize attribute for each VAO that it is linked to
                for vao_id in vao_ids {
                    let vao = self
                        .vertex_array_objects
                        .get(vao_id)
                        .ok_or(CreateAttributeError::VAONotFound)?;

                    gl.bind_vertex_array(Some(vao));
                    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&webgl_buffer));
                    gl.enable_vertex_attrib_array(*attribute_location);
                    // create callback is expected to initialize its associated attribute
                    // with a call to vertexAttribPointer,
                    // which is saved in the associated VAO
                    attribute_link.create_attribute(
                        gl.clone(),
                        now,
                        webgl_buffer.clone(),
                        attribute_location.into(),
                    );
                    gl.bind_vertex_array(None);
                    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
                }
            }

            let attribute = Attribute::new(
                vao_ids.to_vec(),
                buffer_id.clone(),
                attribute_id.clone(),
                webgl_buffer.clone(),
                attribute_location.into(),
            );

            self.attributes.insert(attribute_id, attribute);
        }

        Ok(self)
    }

    /// Creates a WebGL texture for each Texture that was supplied using the create_texture callback
    fn create_textures(&mut self) -> Result<&mut Self, CreateTextureError> {
        let gl = self.gl.as_ref().ok_or(CreateTextureError::NoContext)?;
        let now = Self::now();

        for texture_link in &self.texture_links {
            let texture_id = texture_link.texture_id().clone();
            let webgl_texture = texture_link.create_texture(gl.clone(), now);
            let texture = Texture::new(texture_id.clone(), webgl_texture);

            self.textures.insert(texture_id, texture);
        }

        Ok(self)
    }

    /// Creates a WebGL Framebuffer for each FramebufferLink that was supplied using the callback
    fn create_framebuffers(&mut self) -> Result<&mut Self, CreateBufferError> {
        let gl = self.gl.as_ref().ok_or(CreateBufferError::NoContext)?;
        let now = Self::now();
        let _user_ctx = self.user_ctx.clone();

        for framebuffer_link in &self.framebuffer_links {
            let framebuffer_id = framebuffer_link.framebuffer_id().clone();
            let webgl_texture = framebuffer_link
                .texture_id()
                .and_then(|texture_id| self.textures.get(&texture_id))
                .map(|texture| texture.webgl_texture())
                .map(Clone::clone);

            let webgl_framebuffer =
                framebuffer_link.create_framebuffer(gl.clone(), now, webgl_texture);
            let framebuffer = Framebuffer::new(framebuffer_id.clone(), webgl_framebuffer);

            self.framebuffers.insert(framebuffer_id, framebuffer);
        }

        Ok(self)
    }

    /// Finds every uniform's position in its corresponding program and builds a wrapper for it
    fn create_uniforms(&mut self) -> Result<&mut Self, CreateUniformError> {
        for uniform_link in self.uniform_links.iter() {
            let uniform_id = uniform_link.uniform_id().clone();
            let uniform = self.create_uniform(uniform_link)?;
            self.uniforms.insert(uniform_id, uniform);
        }

        Ok(self)
    }

    fn link_program(
        &self,
        program_link: &ProgramLink<ProgramId, VertexShaderId, FragmentShaderId>,
    ) -> Result<WebGlProgram, LinkProgramError> {
        let gl = self.gl.as_ref().ok_or(LinkProgramError::NoContext)?;

        let vertex_shader_id = program_link.vertex_shader_id();
        let vertex_shader = self
            .vertex_shaders
            .get(vertex_shader_id)
            .ok_or(LinkProgramError::VertexShaderNotFound)?;

        let fragment_shader_id = program_link.fragment_shader_id();
        let fragment_shader = self
            .fragment_shaders
            .get(fragment_shader_id)
            .ok_or(LinkProgramError::FragmentShaderNotFound)?;

        // @todo - make this not have to clone the slice
        let transform_feedback_varyings = program_link.transform_feedback_varyings().to_vec();

        let webgl_program = gl.create_program().ok_or(LinkProgramError::NoProgram)?;

        // assign attribute locations
        for (attribute_id, attribute_location) in self.attribute_locations.iter() {
            gl.bind_attrib_location(&webgl_program, *attribute_location, &attribute_id.name());
        }

        gl.attach_shader(&webgl_program, vertex_shader);
        gl.attach_shader(&webgl_program, fragment_shader);

        if !transform_feedback_varyings.is_empty() {
            let varyings_js_value = JsValue::from_serde(&transform_feedback_varyings)
                .map_err(|_| LinkProgramError::CouldNotConvertVaryingsToArray)?;
            gl.transform_feedback_varyings(
                &webgl_program,
                &varyings_js_value,
                WebGl2RenderingContext::INTERLEAVED_ATTRIBS,
            )
        }

        gl.link_program(&webgl_program);

        if gl
            .get_program_parameter(&webgl_program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(webgl_program)
        } else {
            let inner_error = match gl.get_program_info_log(&webgl_program) {
                Some(known_error) => LinkProgramError::KnownError(known_error),
                None => LinkProgramError::UnknownError,
            };
            Err(inner_error)?
        }
    }

    /// Gets current DOMHighResTimeStamp from performance.now()
    ///
    /// WebGL is limited to an f32, so using performance.now() (for now) to limit the size of the f64
    fn now() -> f64 {
        window().unwrap().performance().unwrap().now()
    }

    /// Takes the string source of a shader and compiles to using the current WebGL2RenderingContext
    fn compile_shader<ShaderId: Id>(
        &self,
        shader_id: ShaderId,
        shader_type: ShaderType,
        source: &str,
    ) -> Result<WebGlShader, CompileShaderError> {
        let gl = self.gl.as_ref().ok_or(CompileShaderError::NoContext {
            shader_id: format!("{shader_id:#?}"),
        })?;

        let shader =
            gl.create_shader(shader_type.into())
                .ok_or(CompileShaderError::NoShaderReturned {
                    shader_id: format!("{shader_id:#?}"),
                })?;

        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);

        if gl
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            let inner_error = match gl.get_shader_info_log(&shader) {
                Some(known_error) => CompileShaderError::KnownError {
                    shader_id: format!("{shader_id:#?}"),
                    error: known_error,
                },
                None => CompileShaderError::UnknownError {
                    shader_id: format!("{shader_id:#?}"),
                },
            };
            Err(inner_error)?
        }
    }
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id,
        AttributeId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        TransformFeedbackId: Id,
        VertexArrayObjectId: Id,
        UserCtx: Clone,
    > Default
    for RendererBuilder<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        TransformFeedbackId,
        VertexArrayObjectId,
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
            attribute_links: Default::default(),
            attributes: Default::default(),
            vertex_array_object_links: Default::default(),
            vertex_array_objects: Default::default(),
            transform_feedbacks: Default::default(),
            transform_feedback_links: Default::default(),
            get_context_callback: Default::default(),
            attribute_locations: Default::default(),
        }
    }
}
