use crate::{
    AttributeJs, BufferJs, FramebufferJs, RendererJsBuilder, RendererHandleJs, TextureJs,
    UniformJs, Renderer,
};
use js_sys::{Array, Map, Object};
use std::ops::{Deref, DerefMut};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlTransformFeedback,
    WebGlVertexArrayObject,
};

/// Wrapper around `Renderer` to make it callable from JavaScript.
///
/// Types are adjusted to only use JavaScript-compatible types and no generics.
pub type RendererJsInner = Renderer<
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

#[wasm_bindgen(js_name = Renderer)]
pub struct RendererJs(RendererJsInner);

#[wasm_bindgen(js_class = Renderer)]
impl RendererJs {
    pub fn builder() -> RendererJsBuilder {
        RendererJsBuilder::default()
    }

    pub fn canvas(&self) -> HtmlCanvasElement {
        self.deref().canvas().clone()
    }

    pub fn gl(&self) -> WebGl2RenderingContext {
        self.deref().gl().clone()
    }

    pub fn fragment_shader(&self, fragment_shader_id: String) -> Option<WebGlShader> {
        self.deref()
            .fragment_shader(&fragment_shader_id)
            .map(Clone::clone)
    }

    pub fn fragment_shaders(&self) -> Map {
        let map = Map::new();

        for (key, value) in self.deref().fragment_shaders().iter() {
            map.set(&JsValue::from_str(key), value.as_ref());
        }

        map
    }

    pub fn vertex_shader(&self, vertex_shader_id: String) -> Option<WebGlShader> {
        self.deref()
            .vertex_shader(&vertex_shader_id)
            .map(Clone::clone)
    }

    pub fn vertex_shaders(&self) -> Map {
        let map = Map::new();

        for (key, value) in self.deref().vertex_shaders().iter() {
            map.set(&JsValue::from_str(key), value.as_ref());
        }

        map
    }

    pub fn program(&self, program_id: String) -> Option<WebGlProgram> {
        self.deref().program(&program_id).map(Clone::clone)
    }

    pub fn programs(&self) -> Map {
        let map = Map::new();

        for (key, value) in self.deref().programs().iter() {
            map.set(&JsValue::from_str(key), value.as_ref());
        }

        map
    }

    pub fn uniform(&self, uniform_id: String) -> Option<UniformJs> {
        self.deref().uniform(&uniform_id).map(Into::into)
    }

    pub fn uniforms(&self) -> Map {
        let map = Map::new();

        for (key, uniform) in self.deref().uniforms().iter() {
            let js_uniform: UniformJs = uniform.into();
            map.set(&JsValue::from_str(key), &js_uniform.into());
        }

        map
    }

    pub fn buffer(&self, buffer_id: String) -> Option<BufferJs> {
        self.deref().buffer(&buffer_id).map(Into::into)
    }

    pub fn buffers(&self) -> Map {
        let map = Map::new();

        for (key, buffer) in self.deref().buffers().iter() {
            let js_buffer: BufferJs = buffer.into();
            map.set(&JsValue::from_str(key), &js_buffer.into());
        }

        map
    }

    pub fn attribute(&self, attribute_id: String) -> Option<AttributeJs> {
        self.deref().attribute(&attribute_id).map(Into::into)
    }

    pub fn attributes(&self) -> Map {
        let map = Map::new();

        for (key, value) in self.deref().attributes().iter() {
            let attribute: AttributeJs = value.into();
            map.set(&JsValue::from_str(key), &attribute.into());
        }

        map
    }

    pub fn texture(&self, texture_id: String) -> Option<TextureJs> {
        self.deref().texture(&texture_id).map(Into::into)
    }

    pub fn textures(&self) -> Map {
        let map = Map::new();

        for (key, texture) in self.deref().textures().iter() {
            let js_texture: TextureJs = texture.into();
            map.set(&JsValue::from_str(key), &js_texture.into());
        }

        map
    }

    pub fn textures_by_id(&self, texture_ids: Array) -> Array {
        let string_vec: Vec<String> = js_sys::try_iter(texture_ids.as_ref())
            .unwrap()
            .expect("textures_by_id should be passed an array of strings")
            .into_iter()
            .map(|el| {
                JsValue::as_string(&el.expect(
                    "Each element in the array passed to textures_by_id should be a string",
                ))
                .unwrap()
            })
            .collect();

        let textures: Vec<JsValue> = self
            .deref()
            .textures_by_id(string_vec)
            .iter()
            .map(|texture| {
                let js_texture: TextureJs = (*texture).into();
                js_texture.into()
            })
            .collect();

        let array = Array::from_iter(textures);

        array
    }

    pub fn framebuffer(&self, framebuffer_id: String) -> Option<FramebufferJs> {
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

    pub fn user_ctx(&self) -> Option<Object> {
        self.deref().user_ctx().map(Clone::clone)
    }

    pub fn use_program(&self, program_id: String) {
        self.deref().use_program(&program_id);
    }

    pub fn use_vao(&self, vao_id: String) {
        self.deref().use_vao(&vao_id);
    }

    pub fn update_uniform(&self, uniform_id: String) {
        self.deref().update_uniform(&uniform_id);
    }

    pub fn update_uniforms(&self) {
        self.deref().update_uniforms();
    }

    pub fn render(&self) {
        self.deref().render();
    }

    pub fn save_image(&self) {
        self.deref().save_image();
    }

    pub fn into_renderer_handle(self) -> RendererHandleJs {
        self.into()
    }
}

impl RendererJs {
    pub fn inner(self) -> RendererJsInner {
        self.0
    }
}

impl Deref for RendererJs {
    type Target = RendererJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RendererJs {
    fn deref_mut(&mut self) -> &mut RendererJsInner {
        &mut self.0
    }
}

impl From<RendererJsInner> for RendererJs {
    fn from(js_renderer_inner: RendererJsInner) -> Self {
        Self(js_renderer_inner)
    }
}
