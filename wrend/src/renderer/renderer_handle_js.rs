use crate::{RendererHandle, RendererJs};
use js_sys::{Function, Object};
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::wasm_bindgen;

pub type RendererHandleJsInner = RendererHandle<
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

#[wasm_bindgen(js_name = RendererHandle)]
pub struct RendererHandleJs(RendererHandleJsInner);

#[wasm_bindgen(js_class = RendererHandle)]
impl RendererHandleJs {
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
    pub fn set_animation_callback(&mut self, animation_callback: Option<Function>) {
        self.deref_mut().set_animation_callback(animation_callback);
    }

    #[wasm_bindgen(js_name = startRecording)]
    pub fn start_recording(&self) {
        self.deref().start_recording();
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
}

impl From<RendererHandleJsInner> for RendererHandleJs {
    fn from(js_renderer_handle_inner: RendererHandleJsInner) -> Self {
        Self(js_renderer_handle_inner)
    }
}

impl From<&RendererHandleJsInner> for RendererHandleJs {
    fn from(js_renderer_handle_inner: &RendererHandleJsInner) -> Self {
        Self(js_renderer_handle_inner.to_owned())
    }
}

impl From<RendererJs> for RendererHandleJs {
    fn from(js_renderer: RendererJs) -> Self {
        Self(js_renderer.inner().into())
    }
}

impl Deref for RendererHandleJs {
    type Target = RendererHandleJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RendererHandleJs {
    fn deref_mut(&mut self) -> &mut RendererHandleJsInner {
        &mut self.0
    }
}
