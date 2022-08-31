use crate::{
    AttributeLinkJs, BufferLinkJs, FramebufferLinkJs, ProgramLinkJs, RendererBuilder, RendererJs,
    TextureJs, TextureLinkJs, TransformFeedbackLinkJs, UniformLinkJs,
};
use js_sys::{Function, Object};
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlCanvasElement;

/// Wrapper around `Renderer` to make it callable from JavaScript.
///
/// Types are adjusted to only use JavaScript-compatible types and no generics.
type RendererJsBuilderInner = RendererBuilder<
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    Object,
>;

/// See [RendererBuilder](crate::RendererBuilder)
#[wasm_bindgen(js_name = RendererBuilder)]
pub struct RendererJsBuilder(RendererJsBuilderInner);

#[wasm_bindgen(js_class = RendererBuilder)]
impl RendererJsBuilder {
    pub fn texture(&self, texture_id: String) -> Option<TextureJs> {
        self.deref().texture(&texture_id).map(Into::into)
    }

    #[wasm_bindgen(js_name = setCanvas)]
    pub fn set_canvas(mut self, canvas: HtmlCanvasElement) -> Self {
        self.deref_mut().set_canvas(canvas);
        self
    }

    #[wasm_bindgen(js_name = addFragmentShaderSrc)]
    pub fn add_fragment_shader_src(mut self, id: String, fragment_shader_src: String) -> Self {
        self.deref_mut()
            .add_fragment_shader_src(id, fragment_shader_src);
        self
    }

    #[wasm_bindgen(js_name = addVertexShaderSrc)]
    pub fn add_vertex_shader_src(mut self, id: String, vertex_shader_src: String) -> Self {
        self.deref_mut()
            .add_vertex_shader_src(id, vertex_shader_src);
        self
    }

    #[wasm_bindgen(js_name = addProgramLink)]
    pub fn add_program_link(mut self, program_link: ProgramLinkJs) -> Self {
        self.deref_mut().add_program_link(program_link);
        self
    }

    #[wasm_bindgen(js_name = setRenderCallback)]
    pub fn set_render_callback(mut self, render_callback: Function) -> Self {
        self.deref_mut().set_render_callback(render_callback);
        self
    }

    #[wasm_bindgen(js_name = setUserCtx)]
    pub fn set_user_ctx(mut self, ctx: Object) -> Self {
        self.deref_mut().set_user_ctx(ctx);
        self
    }

    #[wasm_bindgen(js_name = addUniformLink)]
    pub fn add_uniform_link(mut self, uniform_link: UniformLinkJs) -> Self {
        self.deref_mut().add_uniform_link(uniform_link);

        self
    }

    #[wasm_bindgen(js_name = addBufferLink)]
    pub fn add_buffer_link(mut self, buffer_link: BufferLinkJs) -> Self {
        self.deref_mut().add_buffer_link(buffer_link);
        self
    }

    #[wasm_bindgen(js_name = addAttributeLink)]
    pub fn add_attribute_link(mut self, attribute_link: AttributeLinkJs) -> Self {
        self.deref_mut().add_attribute_link(attribute_link);
        self
    }

    #[wasm_bindgen(js_name = addTextureLink)]
    pub fn add_texture_link(mut self, texture_link: TextureLinkJs) -> Self {
        self.deref_mut().add_texture_link(texture_link);
        self
    }

    #[wasm_bindgen(js_name = addFramebufferLink)]
    pub fn add_framebuffer_link(mut self, framebuffer_link: FramebufferLinkJs) -> Self {
        self.deref_mut().add_framebuffer_link(framebuffer_link);
        self
    }

    #[wasm_bindgen(js_name = addTransformFeedbackLink)]
    pub fn add_transform_feedback_link(
        mut self,
        transform_feedback_link: TransformFeedbackLinkJs,
    ) -> Self {
        self.deref_mut()
            .add_transform_feedback_link(transform_feedback_link);
        self
    }

    #[wasm_bindgen(js_name = addVAOLink)]
    pub fn add_vao_link(mut self, vertex_array_object_id: String) -> Self {
        self.deref_mut().add_vao_link(vertex_array_object_id);
        self
    }

    #[wasm_bindgen(js_name = setGetContextCallback)]
    pub fn set_get_context_callback(mut self, get_context_callback: Function) -> Self {
        self.deref_mut()
            .set_get_context_callback(get_context_callback);
        self
    }

    pub fn build(self) -> Result<RendererJs, String> {
        self.0
            .build()
            .map(Into::into)
            .map_err(|err| err.to_string())
    }

    pub fn default() -> Self {
        Self(RendererBuilder::default())
    }
}

impl Deref for RendererJsBuilder {
    type Target = RendererJsBuilderInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RendererJsBuilder {
    fn deref_mut(&mut self) -> &mut RendererJsBuilderInner {
        &mut self.0
    }
}
