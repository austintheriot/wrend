use crate::{RendererJs, RendererHandle};
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
