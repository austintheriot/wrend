use crate::{JsRenderer, RendererHandle};
use js_sys::{Function, Object};
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::wasm_bindgen;

pub type JsRendererHandleInner = RendererHandle<
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
pub struct JsRendererHandle(JsRendererHandleInner);

#[wasm_bindgen(js_class = RendererHandle)]
impl JsRendererHandle {
    pub fn initialize_recorder(&mut self) {
        self.deref_mut().initialize_recorder();
    }

    pub fn start_animating(&self) {
        self.deref().start_animating();
    }

    pub fn stop_animating(&self) {
        self.deref().stop_animating();
    }

    pub fn set_animation_callback(&mut self, animation_callback: Option<Function>) {
        self.deref_mut().set_animation_callback(animation_callback);
    }

    pub fn start_recording(&self) {
        self.deref().start_recording();
    }

    pub fn stop_recording(&self) {
        self.deref().stop_recording();
    }

    pub fn recorder_initialized(&self) -> bool {
        self.deref().recorder_initialized()
    }

    pub fn is_animating(&self) -> bool {
        self.deref().is_animating()
    }

    pub fn is_recording(&self) -> bool {
        self.deref().is_recording()
    }
}

impl From<JsRendererHandleInner> for JsRendererHandle {
    fn from(js_renderer_handle_inner: JsRendererHandleInner) -> Self {
        Self(js_renderer_handle_inner)
    }
}

impl From<&JsRendererHandleInner> for JsRendererHandle {
    fn from(js_renderer_handle_inner: &JsRendererHandleInner) -> Self {
        Self(js_renderer_handle_inner.to_owned())
    }
}

impl From<JsRenderer> for JsRendererHandle {
    fn from(js_renderer: JsRenderer) -> Self {
        Self(js_renderer.inner().into())
    }
}

impl Deref for JsRendererHandle {
    type Target = JsRendererHandleInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JsRendererHandle {
    fn deref_mut(&mut self) -> &mut JsRendererHandleInner {
        &mut self.0
    }
}
