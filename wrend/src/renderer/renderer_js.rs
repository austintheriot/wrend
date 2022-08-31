use crate::{
    utils, AttributeJs, BufferJs, FramebufferJs, Id, IdName, IntoJsWrapper, Renderer,
    RendererHandleJs, RendererJsBuilder, TextureJs, UniformJs,
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

    #[wasm_bindgen(js_name = fragmentShader)]
    pub fn fragment_shader(&self, fragment_shader_id: String) -> Option<WebGlShader> {
        self.deref()
            .fragment_shader(&fragment_shader_id)
            .map(Clone::clone)
    }

    #[wasm_bindgen(js_name = fragmentShaders)]
    pub fn fragment_shaders(&self) -> Map {
        let map = Map::new();

        for (key, value) in self.deref().fragment_shaders().iter() {
            map.set(&JsValue::from_str(key), value.as_ref());
        }

        map
    }

    #[wasm_bindgen(js_name = vertexShader)]
    pub fn vertex_shader(&self, vertex_shader_id: String) -> Option<WebGlShader> {
        self.deref()
            .vertex_shader(&vertex_shader_id)
            .map(Clone::clone)
    }

    #[wasm_bindgen(js_name = vertexShaders)]
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

    #[wasm_bindgen(js_name = textureById)]
    pub fn textures_by_id(&self, texture_ids: Array) -> Array {
        let string_vec: Vec<String> = utils::js_array_to_vec_strings(texture_ids);

        let textures: Vec<JsValue> = self
            .deref()
            .textures_by_id(string_vec)
            .iter()
            .map(|texture| {
                let js_texture: TextureJs = (*texture).into();
                js_texture.into()
            })
            .collect();

        Array::from_iter(textures)
    }

    pub fn framebuffer(&self, framebuffer_id: String) -> Option<FramebufferJs> {
        self.deref().framebuffer(&framebuffer_id).map(Into::into)
    }

    #[wasm_bindgen(js_name = transformFeedback)]
    pub fn transform_feedback(
        &self,
        transform_feedback_id: String,
    ) -> Option<WebGlTransformFeedback> {
        self.deref()
            .transform_feedback(&transform_feedback_id)
            .map(Clone::clone)
    }

    #[wasm_bindgen(js_name = VAO)]
    pub fn vao(&self, vao_id: String) -> Option<WebGlVertexArrayObject> {
        self.deref().vao(&vao_id).map(Clone::clone)
    }

    #[wasm_bindgen(js_name = userCtx)]
    pub fn user_ctx(&self) -> Option<Object> {
        self.deref().user_ctx().map(Clone::clone)
    }

    #[wasm_bindgen(js_name = useProgram)]
    pub fn use_program(&self, program_id: String) {
        self.deref().use_program(&program_id);
    }

    #[wasm_bindgen(js_name = useVAO)]
    pub fn use_vao(&self, vao_id: String) {
        self.deref().use_vao(&vao_id);
    }

    #[wasm_bindgen(js_name = updateUniform)]
    pub fn update_uniform(&self, uniform_id: String) {
        self.deref().update_uniform(&uniform_id);
    }

    #[wasm_bindgen(js_name = updateUniforms)]
    pub fn update_uniforms(&self) {
        self.deref().update_uniforms();
    }

    pub fn render(&self) {
        self.deref().render();
    }

    #[wasm_bindgen(js_name = saveImage)]
    pub fn save_image(&self) {
        self.deref().save_image();
    }

    #[wasm_bindgen(js_name = intoRendererHandle)]
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

impl IntoJsWrapper for RendererJsInner {
    type Result = RendererJs;

    fn into_js_wrapper(self) -> Self::Result {
        self.into()
    }
}

impl From<&RendererJsInner> for RendererJs {
    fn from(js_renderer_inner: &RendererJsInner) -> Self {
        Self(js_renderer_inner.to_owned())
    }
}

impl IntoJsWrapper for &RendererJsInner {
    type Result = RendererJs;

    fn into_js_wrapper(self) -> Self::Result {
        self.into()
    }
}
