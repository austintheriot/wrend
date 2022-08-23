use crate::RendererHandle;
use js_sys::Object;
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

#[wasm_bindgen]
pub struct JsRendererHandle(JsRendererHandleInner);

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
