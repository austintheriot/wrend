use crate::Id;

use super::buffer_create_context::BufferCreateContext;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

pub type BufferCreateCallback<UserCtx> = Rc<dyn Fn(&BufferCreateContext<UserCtx>) -> WebGlBuffer>;

#[derive(Clone)]
pub struct BufferLink<BufferId: Id, UserCtx: 'static> {
    buffer_id: BufferId,
    create_buffer_callback: BufferCreateCallback<UserCtx>,
}

impl<BufferId: Id, UserCtx> BufferLink<BufferId, UserCtx> {
    pub fn new(
        buffer_id: impl Into<BufferId>,
        create_buffer_callback: BufferCreateCallback<UserCtx>,
    ) -> Self {
        Self {
            buffer_id: buffer_id.into(),
            create_buffer_callback,
        }
    }

    pub fn buffer_id(&self) -> &BufferId {
        &self.buffer_id
    }

    pub fn create_buffer(
        &self,
        gl: &WebGl2RenderingContext,
        now: f64,
        user_ctx: Option<&UserCtx>,
    ) -> WebGlBuffer {
        let framebuffer_create_context = BufferCreateContext::new(gl, now, user_ctx);
        (self.create_buffer_callback)(&framebuffer_create_context)
    }
}

impl<BufferId: Id, UserCtx> Debug for BufferLink<BufferId, UserCtx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferLink")
            .field("buffer_id", &self.buffer_id)
            .finish()
    }
}

impl<BufferId: Id, UserCtx> Hash for BufferLink<BufferId, UserCtx> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.buffer_id.hash(state);
    }
}

impl<BufferId: Id, UserCtx> PartialEq for BufferLink<BufferId, UserCtx> {
    fn eq(&self, other: &Self) -> bool {
        self.buffer_id == other.buffer_id
    }
}

impl<BufferId: Id, UserCtx> Eq for BufferLink<BufferId, UserCtx> {}
