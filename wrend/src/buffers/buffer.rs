use crate::{BufferJs, BufferJsInner, Id};
use std::fmt::Debug;
use std::hash::Hash;
use wasm_bindgen::JsValue;
use web_sys::WebGlBuffer;

#[derive(Clone)]
pub struct Buffer<BufferId: Id> {
    buffer_id: BufferId,
    webgl_buffer: WebGlBuffer,
}

impl<BufferId: Id> Buffer<BufferId> {
    // @todo move into builder pattern ?
    pub fn new(buffer_id: BufferId, webgl_buffer: WebGlBuffer) -> Self {
        Self {
            buffer_id,
            webgl_buffer,
        }
    }

    pub fn buffer_id(&self) -> &BufferId {
        &self.buffer_id
    }

    pub fn webgl_buffer(&self) -> &WebGlBuffer {
        &self.webgl_buffer
    }
}

impl<BufferId: Id> Debug for Buffer<BufferId> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Buffer")
            .field("buffer_id", &self.buffer_id)
            .field("webgl_buffer", &self.webgl_buffer)
            .finish()
    }
}
impl<BufferId: Id> Hash for Buffer<BufferId> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.buffer_id.hash(state);
    }
}

impl<BufferId: Id> PartialEq for Buffer<BufferId> {
    fn eq(&self, other: &Self) -> bool {
        self.buffer_id == other.buffer_id && self.webgl_buffer == other.webgl_buffer
    }
}

impl<BufferId: Id> Eq for Buffer<BufferId> {}

impl From<BufferJsInner> for JsValue {
    fn from(buffer: BufferJsInner) -> Self {
        let js_buffer: BufferJs = buffer.into();
        js_buffer.into()
    }
}
