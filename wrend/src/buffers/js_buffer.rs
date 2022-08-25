use crate::Buffer;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::WebGlBuffer;

pub type JsBufferInner = Buffer<String>;

#[wasm_bindgen]
pub struct JsBuffer(JsBufferInner);

#[wasm_bindgen]
impl JsBuffer {
    pub fn buffer_id(&self) -> String {
        self.deref().buffer_id().to_owned()
    }

    pub fn webgl_buffer(&self) -> WebGlBuffer {
        self.deref().webgl_buffer().to_owned()
    }
}

impl Deref for JsBuffer {
    type Target = JsBufferInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JsBuffer {
    fn deref_mut(&mut self) -> &mut JsBufferInner {
        &mut self.0
    }
}

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
