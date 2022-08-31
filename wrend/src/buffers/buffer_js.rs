use crate::Buffer;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::WebGlBuffer;

pub type BufferJsInner = Buffer<String>;

#[wasm_bindgen(js_name = Buffer)]
pub struct BufferJs(BufferJsInner);

#[wasm_bindgen(js_class = Buffer)]
impl BufferJs {
    #[wasm_bindgen(js_name = bufferId)]
    pub fn buffer_id(&self) -> String {
        self.deref().buffer_id().to_owned()
    }

    #[wasm_bindgen(js_name = webglBuffer)]
    pub fn webgl_buffer(&self) -> WebGlBuffer {
        self.deref().webgl_buffer().to_owned()
    }
}

impl Deref for BufferJs {
    type Target = BufferJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BufferJs {
    fn deref_mut(&mut self) -> &mut BufferJsInner {
        &mut self.0
    }
}

impl From<BufferJsInner> for BufferJs {
    fn from(js_buffer_inner: BufferJsInner) -> Self {
        Self(js_buffer_inner)
    }
}

impl From<&BufferJsInner> for BufferJs {
    fn from(js_buffer_inner: &BufferJsInner) -> Self {
        Self(js_buffer_inner.to_owned())
    }
}
