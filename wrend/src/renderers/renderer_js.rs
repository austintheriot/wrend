use crate::{
    AnimationCallbackJs, AttributeJs, BufferJs, Callback, FramebufferJs, RenderCallbackJs,
    Renderer, RendererDataBuilderJs, RendererDataJs, TextureJs, UniformJs,
};
use js_sys::Object;
use log::error;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlTransformFeedback,
    WebGlVertexArrayObject,
};

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
    #[wasm_bindgen(js_name = initializeRecorder)]
    pub fn initialize_recorder(&mut self) {
        self.deref_mut().initialize_recorder();
    }

    #[wasm_bindgen(js_name = startAnimating)]
    pub fn start_animating(&self) {
        self.deref().start_animating();
    }

    #[wasm_bindgen(js_name = stopAnimating)]
    pub fn stop_animating(&self) {
        self.deref().stop_animating();
    }

    #[wasm_bindgen(js_name = setAnimationCallback)]
    pub fn set_animation_callback(&mut self, animation_callback: Option<AnimationCallbackJs>) {
        self.deref_mut().set_animation_callback(animation_callback);
    }

    #[wasm_bindgen(js_name = startRecording)]
    pub fn start_recording(&mut self) {
        self.deref_mut().start_recording();
    }

    #[wasm_bindgen(js_name = stopRecording)]
    pub fn stop_recording(&self) {
        self.deref().stop_recording();
    }

    #[wasm_bindgen(js_name = recorderInitialized)]
    pub fn recorder_initialized(&self) -> bool {
        self.deref().recorder_initialized()
    }

    #[wasm_bindgen(js_name = isAnimating)]
    pub fn is_animating(&self) -> bool {
        self.deref().is_animating()
    }

    #[wasm_bindgen(js_name = isRecording)]
    pub fn is_recording(&self) -> bool {
        self.deref().is_recording()
    }

    #[wasm_bindgen(js_name = rendererData)]
    pub fn renderer_data(&self) -> RendererDataJs {
        self.deref().renderer_data().into()
    }

    pub fn builder() -> RendererDataBuilderJs {
        RendererDataBuilderJs::default()
    }

    pub fn canvas(&self) -> HtmlCanvasElement {
        self.deref().borrow().canvas().to_owned()
    }

    pub fn gl(&self) -> WebGl2RenderingContext {
        self.deref().borrow().deref().gl().to_owned()
    }

    #[wasm_bindgen(js_name = fragmentShader)]
    pub fn fragment_shader(&self, fragment_shader_id: String) -> Option<WebGlShader> {
        self.deref()
            .borrow()
            .fragment_shader(&fragment_shader_id)
            .map(Clone::clone)
    }

    #[wasm_bindgen(js_name = vertexShader)]
    pub fn vertex_shader(&self, vertex_shader_id: String) -> Option<WebGlShader> {
        self.deref()
            .borrow()
            .vertex_shader(&vertex_shader_id)
            .map(Clone::clone)
    }

    pub fn program(&self, program_id: String) -> Option<WebGlProgram> {
        self.deref().borrow().program(&program_id).map(Clone::clone)
    }

    pub fn uniform(&self, uniform_id: String) -> Option<UniformJs> {
        self.deref()
            .borrow()
            .uniform(&uniform_id)
            .map(Clone::clone)
            .map(Into::into)
    }

    pub fn buffer(&self, buffer_id: String) -> Option<BufferJs> {
        self.deref()
            .borrow()
            .buffer(&buffer_id)
            .map(Clone::clone)
            .map(Into::into)
    }

    pub fn attribute(&self, attribute_id: String) -> Option<AttributeJs> {
        self.deref()
            .borrow()
            .attribute(&attribute_id)
            .map(Clone::clone)
            .map(Into::into)
    }

    pub fn texture(&self, texture_id: String) -> Option<TextureJs> {
        self.deref()
            .borrow()
            .texture(&texture_id)
            .map(Clone::clone)
            .map(Into::into)
    }

    pub fn framebuffer(&self, framebuffer_id: String) -> Option<FramebufferJs> {
        self.deref()
            .borrow()
            .framebuffer(&framebuffer_id)
            .map(Clone::clone)
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

    pub fn render(&self) {
        // does not deref() into the inner `RendererData` here, because it is more efficient
        // to keep this type as-is and pass in itself as a reference to the JavaScript function
        let renderer_data_js = self.renderer_data();
        let render_callback = renderer_data_js.render_callback();
        match &*render_callback {
            Callback::Rust(rust_callback) => {
                let renderer_data = renderer_data_js.into_inner();
                (rust_callback)(&renderer_data.borrow());
            }
            Callback::Js(js_callback) => {
                let js_callback = js_callback.deref();
                let js_value: JsValue = renderer_data_js.into();
                let result = js_callback.call1(&JsValue::NULL, &js_value);
                if let Err(error) = result {
                    error!("Error occurred while calling JavaScript `render` callback: {error:?}");
                }
            }
        }
    }

    #[wasm_bindgen(js_name = saveImage)]
    pub fn save_image(&self) {
        self.deref().borrow().save_image()
    }

    #[wasm_bindgen(js_name = renderCallback)]
    pub fn render_callback(&self) -> Option<RenderCallbackJs> {
        self.deref()
            .borrow()
            .render_callback()
            .js()
            .map(|callback| callback.deref().clone())
    }
}

impl From<RendererJsInner> for RendererJs {
    fn from(js_renderer_handle_inner: RendererJsInner) -> Self {
        Self(js_renderer_handle_inner)
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
