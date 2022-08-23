use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::{
    Attribute, Buffer, Framebuffer, JsAttribute, JsBuffer, JsFramebuffer, JsRendererHandle,
    JsTexture, JsUniform, Renderer, RendererHandle, Texture, Uniform,
};
use js_sys::Object;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{
    window, HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader,
    WebGlTransformFeedback, WebGlVertexArrayObject,
};

/// Wrapper around `Renderer` to make it callable from JavaScript.
///
/// Types are adjusted to only use JavaScript-compatible types and no generics.
type JsRendererInner = Renderer<
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

#[wasm_bindgen]
pub struct JsRenderer(JsRendererInner);

#[wasm_bindgen]
impl JsRenderer {
    // pub fn builder() -> RendererBuilder<
    //     VertexShaderId,
    //     FragmentShaderId,
    //     ProgramId,
    //     UniformId,
    //     BufferId,
    //     AttributeId,
    //     TextureId,
    //     FramebufferId,
    //     TransformFeedbackId,
    //     VertexArrayObjectId,
    //     UserCtx,
    // > {
    //     RendererBuilder::default()
    // }

    pub fn canvas(&self) -> HtmlCanvasElement {
        self.canvas().clone()
    }

    pub fn gl(&self) -> WebGl2RenderingContext {
        self.gl().clone()
    }

    pub fn fragment_shader(&self, fragment_shader_id: String) -> Option<WebGlShader> {
        self.fragment_shader(fragment_shader_id)
    }

    // pub fn fragment_shaders(&self) -> &HashMap<String, WebGlShader> {
    //     self.0.fragment_shaders()
    // }

    pub fn vertex_shader(&self, vertex_shader_id: String) -> Option<WebGlShader> {
        self.vertex_shader(vertex_shader_id)
    }

    // pub fn vertex_shaders(&self) -> &HashMap<VertexShaderId, WebGlShader> {
    //     &self.vertex_shaders
    // }

    pub fn program(&self, program_id: String) -> Option<WebGlProgram> {
        self.program(program_id)
    }

    // pub fn programs(&self) -> &HashMap<ProgramId, WebGlProgram> {
    //     &self.programs
    // }

    pub fn uniform(&self, uniform_id: String) -> Option<JsUniform> {
        self.uniform(uniform_id)
    }

    // pub fn uniforms(&self) -> &HashMap<UniformId, Uniform<ProgramId, UniformId, UserCtx>> {
    //     &self.uniforms
    // }

    pub fn buffer(&self, buffer_id: String) -> Option<JsBuffer> {
        self.deref().buffer(&buffer_id).map(Into::into)
    }

    // pub fn buffers(&self) -> &HashMap<String, Buffer<String>> {
    //     &self.buffers
    // }

    pub fn attribute(&self, attribute_id: String) -> Option<JsAttribute> {
        self.deref().attribute(&attribute_id).map(Into::into)
    }

    // pub fn attributes(
    //     &self,
    // ) -> &HashMap<AttributeId, Attribute<VertexArrayObjectId, BufferId, AttributeId>> {
    //     &self.attributes
    // }

    pub fn texture(&self, texture_id: String) -> Option<JsTexture> {
        self.deref().texture(&texture_id).map(Into::into)
    }

    // pub fn textures(&self) -> &HashMap<String, Texture<String>> {
    //     &self.textures
    // }

    // pub fn textures_by_id(&self, texture_ids: Vec<String>) -> Vec<JsTexture> {
    //     let mut textures = Vec::with_capacity(texture_ids.len());
    //     for texture_id in texture_ids {
    //         let texture = self.texture(&texture_id);
    //         if let Some(texture) = texture {
    //             textures.push(texture);
    //         }
    //     }
    //     textures
    // }

    pub fn framebuffer(&self, framebuffer_id: String) -> Option<JsFramebuffer> {
        self.deref().framebuffer(&framebuffer_id).map(Into::into)
    }

    pub fn transform_feedback(
        &self,
        transform_feedback_id: String,
    ) -> Option<WebGlTransformFeedback> {
        self.deref()
            .transform_feedback(&transform_feedback_id)
            .map(Clone::clone)
    }

    pub fn vao(&self, vao_id: String) -> Option<WebGlVertexArrayObject> {
        self.deref().vao(&vao_id).map(Clone::clone)
    }

    // @todo - enable ctx to be returned unconditionally (depending on if it's set or not)
    pub fn user_ctx(&self) -> Option<Object> {
        self.user_ctx()
    }

    /// Switches to using new program and its associated VAO
    pub fn use_program(&self, program_id: String) {
        self.deref().use_program(&program_id);
    }

    pub fn use_vao(&self, vao_id: String) {
        self.deref().use_vao(&vao_id);
    }

    /// Updates a single uniform using the previously given update function. If no function was supplied,
    /// then this is a no-op.
    ///
    /// Calls "use_program" on the appropriate program before each uniform's update function (so this is not
    /// necessary to do within the callback itself, unless you need to change programs, for whatever reason).
    pub fn update_uniform(&self, uniform_id: String) {
        self.deref().update_uniform(&uniform_id);
    }

    /// Iterates through all saved uniforms and updates them using their associated update callbacks.
    pub fn update_uniforms(&self) {
        self.deref().update_uniforms();
    }

    pub fn render(&self) {
        self.deref().render();
    }

    pub fn save_image(&self) {
        self.deref().save_image();
    }

    /// Begins the animation process.
    ///
    /// If no animation callback has been provided, then the empty animation callback is run.
    // pub fn into_renderer_handle(self) -> JsRendererHandle {
    //     self.into()
    // }

    /// Gets current DOMHighResTimeStamp from performance.now()
    ///
    /// WebGL is limited to an f32, so using performance.now() (for now) to limit the size of the f64
    fn now() -> f64 {
        window().unwrap().performance().unwrap().now()
    }
}

impl Deref for JsRenderer {
    type Target = JsRendererInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JsRenderer {
    fn deref_mut(&mut self) -> &mut JsRendererInner {
        &mut self.0
    }
}
