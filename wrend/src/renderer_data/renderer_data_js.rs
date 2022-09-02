use crate::{
    utils, AttributeJs, BufferJs, FramebufferJs, RenderCallback, RendererData,
    RendererDataBuilderJs, RendererJs, RendererJsInner, TextureJs, UniformJs,
};
use js_sys::{Array, Map, Object};
use log::error;
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlTransformFeedback,
    WebGlVertexArrayObject,
};

/// Wrapper around `RendererData` to make it callable from JavaScript.
///
/// Types are adjusted to only use JavaScript-compatible types and no generics.
pub type RendererDataJsInner = RendererData<
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

#[wasm_bindgen(js_name = RendererData)]
#[derive(Debug, Clone, PartialEq, Eq)]
// Reference counting the internals here is necessary to be able to
// convert this value into a `JsValue` without cloning its internal data.
pub struct RendererDataJs(Rc<RefCell<RendererDataJsInner>>);

#[wasm_bindgen(js_class = RendererData)]
impl RendererDataJs {
    pub fn builder() -> RendererDataBuilderJs {
        RendererDataBuilderJs::default()
    }

    pub fn canvas(&self) -> HtmlCanvasElement {
        self.deref().borrow().canvas().clone()
    }

    pub fn gl(&self) -> WebGl2RenderingContext {
        self.deref().borrow().gl().clone()
    }

    #[wasm_bindgen(js_name = fragmentShader)]
    pub fn fragment_shader(&self, fragment_shader_id: String) -> Option<WebGlShader> {
        self.deref()
            .borrow()
            .fragment_shader(&fragment_shader_id)
            .map(Clone::clone)
    }

    #[wasm_bindgen(js_name = fragmentShaders)]
    pub fn fragment_shaders(&self) -> Map {
        let map = Map::new();

        for (key, value) in self.deref().borrow().fragment_shaders().iter() {
            map.set(&JsValue::from_str(key), value.as_ref());
        }

        map
    }

    #[wasm_bindgen(js_name = vertexShader)]
    pub fn vertex_shader(&self, vertex_shader_id: String) -> Option<WebGlShader> {
        self.deref()
            .borrow()
            .vertex_shader(&vertex_shader_id)
            .map(Clone::clone)
    }

    #[wasm_bindgen(js_name = vertexShaders)]
    pub fn vertex_shaders(&self) -> Map {
        let map = Map::new();

        for (key, value) in self.deref().borrow().vertex_shaders().iter() {
            map.set(&JsValue::from_str(key), value.as_ref());
        }

        map
    }

    pub fn program(&self, program_id: String) -> Option<WebGlProgram> {
        self.deref().borrow().program(&program_id).map(Clone::clone)
    }

    pub fn programs(&self) -> Map {
        let map = Map::new();

        for (key, value) in self.deref().borrow().programs().iter() {
            map.set(&JsValue::from_str(key), value.as_ref());
        }

        map
    }

    pub fn uniform(&self, uniform_id: String) -> Option<UniformJs> {
        self.deref().borrow().uniform(&uniform_id).map(Into::into)
    }

    pub fn uniforms(&self) -> Map {
        let map = Map::new();

        for (key, uniform) in self.deref().borrow().uniforms().iter() {
            let js_uniform: UniformJs = uniform.into();
            map.set(&JsValue::from_str(key), &js_uniform.into());
        }

        map
    }

    pub fn buffer(&self, buffer_id: String) -> Option<BufferJs> {
        self.deref().borrow().buffer(&buffer_id).map(Into::into)
    }

    pub fn buffers(&self) -> Map {
        let map = Map::new();

        for (key, buffer) in self.deref().borrow().buffers().iter() {
            let js_buffer: BufferJs = buffer.into();
            map.set(&JsValue::from_str(key), &js_buffer.into());
        }

        map
    }

    pub fn attribute(&self, attribute_id: String) -> Option<AttributeJs> {
        self.deref()
            .borrow()
            .attribute(&attribute_id)
            .map(Into::into)
    }

    pub fn attributes(&self) -> Map {
        let map = Map::new();

        for (key, value) in self.deref().borrow().attributes().iter() {
            let attribute: AttributeJs = value.into();
            map.set(&JsValue::from_str(key), &attribute.into());
        }

        map
    }

    pub fn texture(&self, texture_id: String) -> Option<TextureJs> {
        self.deref().borrow().texture(&texture_id).map(Into::into)
    }

    pub fn textures(&self) -> Map {
        let map = Map::new();

        for (key, texture) in self.deref().borrow().textures().iter() {
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
            .borrow()
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
        self.deref()
            .borrow()
            .framebuffer(&framebuffer_id)
            .map(Into::into)
    }

    #[wasm_bindgen(js_name = transformFeedback)]
    pub fn transform_feedback(
        &self,
        transform_feedback_id: String,
    ) -> Option<WebGlTransformFeedback> {
        self.deref()
            .borrow()
            .transform_feedback(&transform_feedback_id)
            .map(Clone::clone)
    }

    #[wasm_bindgen(js_name = VAO)]
    pub fn vao(&self, vao_id: String) -> Option<WebGlVertexArrayObject> {
        self.deref().borrow().vao(&vao_id).map(Clone::clone)
    }

    #[wasm_bindgen(js_name = userCtx)]
    pub fn user_ctx(&self) -> Option<Object> {
        self.deref().borrow().user_ctx().map(Clone::clone)
    }

    #[wasm_bindgen(js_name = useProgram)]
    pub fn use_program(&self, program_id: String) {
        self.deref().borrow().use_program(&program_id);
    }

    #[wasm_bindgen(js_name = useVAO)]
    pub fn use_vao(&self, vao_id: String) {
        self.deref().borrow().use_vao(&vao_id);
    }

    #[wasm_bindgen(js_name = updateUniform)]
    pub fn update_uniform(&self, uniform_id: String) {
        self.deref().borrow().update_uniform(&uniform_id);
    }

    #[wasm_bindgen(js_name = updateUniforms)]
    pub fn update_uniforms(&self) {
        self.deref().borrow().update_uniforms();
    }

    // `render` does not deref to the internal `RendererData` here, because its much less complex (and much faster) to
    // pass `RendererDataJs` as an argument to the `render` function here at this level , rather than converting
    // back into a `RendererDataJs` from within the `RendererData` struct.
    pub fn render(&self) {
        let render_callback = self.deref().borrow().render_callback();
        if let Some(js_callback) = render_callback.b().as_ref() {
            // Internals of `RendererDataJs` are stored behind an `Rc`, so this is a cheap operation
            let js_value: JsValue = self.clone().into();
            if let Err(err) = js_callback.call1(&JsValue::NULL, &js_value) {
                error!("Error occurred while calling JavaScript `render` callback: {err:?}");
            }
        } else {
            error!("`render_js` was called without any `js_callback` to call. This is a no-op.")
        }
    }

    #[wasm_bindgen(js_name = saveImage)]
    pub fn save_image(&self) {
        self.deref().borrow().save_image();
    }

    #[wasm_bindgen(js_name = intoRendererHandle)]
    pub fn into_renderer_handle(self) -> RendererJs {
        self.into()
    }
}

impl RendererDataJs {
    pub fn into_inner(self) -> Rc<RefCell<RendererDataJsInner>> {
        self.0
    }

    pub fn render_callback(
        &self,
    ) -> RenderCallback<
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
    > {
        self.deref().borrow().render_callback()
    }
}

impl Deref for RendererDataJs {
    type Target = Rc<RefCell<RendererDataJsInner>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RendererDataJs {
    fn deref_mut(&mut self) -> &mut Rc<RefCell<RendererDataJsInner>> {
        &mut self.0
    }
}

impl From<RendererDataJsInner> for RendererDataJs {
    fn from(renderer_data_js_inner: RendererDataJsInner) -> Self {
        Self(Rc::new(RefCell::new(renderer_data_js_inner)))
    }
}

impl From<Rc<RefCell<RendererDataJsInner>>> for RendererDataJs {
    fn from(renderer_data_js_inner: Rc<RefCell<RendererDataJsInner>>) -> Self {
        Self(renderer_data_js_inner)
    }
}

impl From<RendererDataJs> for RendererJs {
    fn from(renderer_data_js: RendererDataJs) -> Self {
        let renderer_data = Rc::clone(renderer_data_js.deref());
        let renderer: RendererJsInner = RendererJsInner::new_with_rc_renderer(renderer_data);
        renderer.into()
    }
}
