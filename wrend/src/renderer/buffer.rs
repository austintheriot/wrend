use super::attribute_location::AttributeLocation;
use super::buffer_context::BufferContext;
use super::id::Id;
use super::id_name::IdName;
use std::hash::Hash;
use std::{fmt::Debug, rc::Rc};
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

pub type BufferUpdateCallback<UserCtx> = Rc<dyn Fn(&BufferContext<UserCtx>)>;

pub type BufferShouldUpdateCallback<UserCtx> = Rc<dyn Fn(&BufferContext<UserCtx>) -> bool>;

#[derive(Clone)]
pub struct Buffer<ProgramId: Id, BufferId: Id + IdName, UserCtx> {
    program_id: ProgramId,
    buffer_id: BufferId,
    webgl_buffer: WebGlBuffer,
    attribute_location: AttributeLocation,
    update_callback: BufferUpdateCallback<UserCtx>,
    should_update_callback: BufferShouldUpdateCallback<UserCtx>,
}

impl<ProgramId: Id, BufferId: Id + IdName, UserCtx> Buffer<ProgramId, BufferId, UserCtx> {
    // @todo move into builder pattern
    pub fn new(
        program_id: ProgramId,
        buffer_id: BufferId,
        webgl_buffer: WebGlBuffer,
        attribute_location: AttributeLocation,
        update_callback: BufferUpdateCallback<UserCtx>,
        should_update_callback: BufferShouldUpdateCallback<UserCtx>,
    ) -> Self {
        Self {
            program_id,
            buffer_id,
            webgl_buffer,
            attribute_location,
            update_callback,
            should_update_callback,
        }
    }

    pub fn program_id(&self) -> &ProgramId {
        &self.program_id
    }

    pub fn buffer_id(&self) -> &BufferId {
        &self.buffer_id
    }

    pub fn webgl_buffer(&self) -> &WebGlBuffer {
        &self.webgl_buffer
    }

    pub fn attribute_location(&self) -> &AttributeLocation {
        &self.attribute_location
    }

    pub fn should_update(
        &self,
        gl: &WebGl2RenderingContext,
        now: f64,
        user_ctx: Option<&UserCtx>,
    ) -> bool {
        let ctx = BufferContext::new(
            gl,
            now,
            self.webgl_buffer(),
            *self.attribute_location(),
            user_ctx,
        );
        (self.should_update_callback)(&ctx)
    }

    pub fn update(&self, gl: &WebGl2RenderingContext, now: f64, user_ctx: Option<&UserCtx>) {
        let ctx = BufferContext::new(
            gl,
            now,
            self.webgl_buffer(),
            *self.attribute_location(),
            user_ctx,
        );
        (self.update_callback)(&ctx);
    }
}

impl<ProgramId: Id, BufferId: Id + IdName, UserCtx> Debug for Buffer<ProgramId, BufferId, UserCtx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Buffer")
            .field("program_id", &self.program_id)
            .field("buffer_id", &self.buffer_id)
            .field("buffer", &self.webgl_buffer)
            .field("attribute_location", &self.attribute_location)
            .finish()
    }
}
impl<ProgramId: Id, BufferId: Id + IdName, UserCtx> Hash for Buffer<ProgramId, BufferId, UserCtx> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.buffer_id.hash(state);
    }
}

impl<ProgramId: Id, BufferId: Id + IdName, UserCtx> PartialEq
    for Buffer<ProgramId, BufferId, UserCtx>
{
    fn eq(&self, other: &Self) -> bool {
        self.program_id == other.program_id
            && self.buffer_id == other.buffer_id
            && self.webgl_buffer == other.webgl_buffer
            && self.attribute_location == other.attribute_location
            && Rc::ptr_eq(&self.update_callback, &other.update_callback)
            && Rc::ptr_eq(&self.should_update_callback, &other.should_update_callback)
    }
}

impl<ProgramId: Id, BufferId: Id + IdName, UserCtx> Eq for Buffer<ProgramId, BufferId, UserCtx> {}
