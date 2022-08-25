use crate::Buffer;
use wasm_bindgen::prelude::wasm_bindgen;

pub type JsBufferInner = Buffer<String>;

#[wasm_bindgen]
pub struct JsBuffer(JsBufferInner);

impl From<JsBufferInner> for JsBuffer {
    fn from(js_buffer_inner: JsBufferInner) -> Self {
        Self(js_buffer_inner)
    }
}

impl From<&JsBufferInner> for JsBuffer {
    fn from(js_buffer_inner: &JsBufferInner) -> Self {
        Self(js_buffer_inner.to_owned())
    }
}