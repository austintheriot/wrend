use crate::{
    JsAttribute, JsBuffer, JsFramebuffer, JsRendererBuilder, JsTexture, JsUniform, Renderer,
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
pub type JsRendererInner = Renderer<
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
    pub fn builder() -> JsRendererBuilder {
        JsRendererBuilder::default()
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

    pub fn uniform(&self, uniform_id: String) -> Option<JsUniform> {
        self.deref().uniform(&uniform_id).map(Into::into)
    }

    pub fn uniforms(&self) -> Map {
        let map = Map::new();

        for (key, value) in self.deref().uniforms().iter() {
            map.set(
                &JsValue::from_str(key),
                todo!("Uniform functionality must be implemented for JsUniform"),
            );
        }

        map
    }

    pub fn buffer(&self, buffer_id: String) -> Option<JsBuffer> {
        self.deref().buffer(&buffer_id).map(Into::into)
    }

    pub fn buffers(&self) -> Map {
        let map = Map::new();

        for (key, value) in self.deref().buffers().iter() {
            map.set(
                &JsValue::from_str(key),
                todo!("Buffer functionality must be implemented for JsBuffer"),
            );
        }

        map
    }

    pub fn attribute(&self, attribute_id: String) -> Option<JsAttribute> {
        self.deref().attribute(&attribute_id).map(Into::into)
    }

    pub fn attributes(&self) -> Map {
        let map = Map::new();

        for (key, value) in self.deref().attributes().iter() {
            map.set(
                &JsValue::from_str(key),
                todo!("Attribute functionality must be implemented for JsAttribute"),
            );
        }

        map
    }

    pub fn texture(&self, texture_id: String) -> Option<JsTexture> {
        self.deref().texture(&texture_id).map(Into::into)
    }

    pub fn textures(&self) -> Map {
        let map = Map::new();

        for (key, value) in self.deref().textures().iter() {
            map.set(
                &JsValue::from_str(key),
                todo!("Texture functionality must be implemented for JsTexture"),
            );
        }

        map
    }

    /// Only returns the WebGlTexture for now
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

        let textures = self.deref().textures_by_id(string_vec);

        let array = Array::new();
        for texture in textures {
            array.push(texture.webgl_texture().as_ref());
        }

        array
    }

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

    // Begins the animation process.
    //
    // If no animation callback has been provided, then the empty animation callback is run.
    // pub fn into_renderer_handle(self) -> JsRendererHandle {
    //     self.into()
    // }
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

impl From<JsRendererInner> for JsRenderer {
    fn from(js_renderer_inner: JsRendererInner) -> Self {
        Self(js_renderer_inner)
    }
}
