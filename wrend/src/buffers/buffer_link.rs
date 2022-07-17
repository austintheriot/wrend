use crate::{BufferCreateCallback, Id};

use super::buffer_create_context::BufferCreateContext;
use std::fmt::Debug;
use std::hash::Hash;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

#[derive(Clone)]
pub struct BufferLink<BufferId: Id, UserCtx: Clone + 'static> {
    buffer_id: BufferId,
    buffer_create_callback: BufferCreateCallback<UserCtx>,
}

impl<BufferId: Id, UserCtx: Clone> BufferLink<BufferId, UserCtx> {
    pub fn new(
        buffer_id: impl Into<BufferId>,
        buffer_create_callback: impl Into<BufferCreateCallback<UserCtx>>,
    ) -> Self {
        Self {
            buffer_id: buffer_id.into(),
            buffer_create_callback: buffer_create_callback.into(),
        }
    }

    pub fn buffer_id(&self) -> &BufferId {
        &self.buffer_id
    }

    pub fn create_buffer(
        &self,
        gl: WebGl2RenderingContext,
        now: f64,
        user_ctx: Option<UserCtx>,
    ) -> WebGlBuffer {
        let framebuffer_create_context = BufferCreateContext::new(gl, now, user_ctx);
        (self.buffer_create_callback)(&framebuffer_create_context)
    }
}

impl<BufferId: Id, UserCtx: Clone> Debug for BufferLink<BufferId, UserCtx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferLink")
            .field("buffer_id", &self.buffer_id)
            .finish()
    }
}

impl<BufferId: Id, UserCtx: Clone> Hash for BufferLink<BufferId, UserCtx> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.buffer_id.hash(state);
    }
}

impl<BufferId: Id, UserCtx: Clone> PartialEq for BufferLink<BufferId, UserCtx> {
    fn eq(&self, other: &Self) -> bool {
        self.buffer_id == other.buffer_id
    }
}

impl<BufferId: Id, UserCtx: Clone> Eq for BufferLink<BufferId, UserCtx> {}
