use super::buffer_create_context::BufferCreateContext;
use crate::{BufferCreateCallback, Id};
use std::fmt::Debug;
use std::hash::Hash;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

#[derive(Clone)]
pub struct BufferLink<BufferId: Id> {
    buffer_id: BufferId,
    buffer_create_callback: BufferCreateCallback,
}

impl<BufferId: Id> BufferLink<BufferId> {
    pub fn new(
        buffer_id: impl Into<BufferId>,
        buffer_create_callback: impl Into<BufferCreateCallback>,
    ) -> Self {
        Self {
            buffer_id: buffer_id.into(),
            buffer_create_callback: buffer_create_callback.into(),
        }
    }

    pub fn buffer_id(&self) -> &BufferId {
        &self.buffer_id
    }

    pub fn create_buffer(&self, gl: WebGl2RenderingContext, now: f64) -> WebGlBuffer {
        let buffer_create_context = BufferCreateContext::new(gl, now);
        self.buffer_create_callback
            .call_with_into_js_arg_and_return(&buffer_create_context)
    }
}

impl<BufferId: Id> Debug for BufferLink<BufferId> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferLink")
            .field("buffer_id", &self.buffer_id)
            .finish()
    }
}

impl<BufferId: Id> Hash for BufferLink<BufferId> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.buffer_id.hash(state);
    }
}

impl<BufferId: Id> PartialEq for BufferLink<BufferId> {
    fn eq(&self, other: &Self) -> bool {
        self.buffer_id == other.buffer_id
    }
}

impl<BufferId: Id> Eq for BufferLink<BufferId> {}
