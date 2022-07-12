use super::attribute_location::AttributeLocation;
use super::buffer::{BufferShouldUpdateCallback, BufferUpdateCallback};
use super::create_buffer_context::CreateBufferContext;
use super::id::Id;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

pub type CreateBufferCallback<UserCtx> = Rc<dyn Fn(CreateBufferContext<UserCtx>) -> WebGlBuffer>;

#[derive(Clone)]
pub struct BufferLink<ProgramId, BufferId, UserCtx>
where
    ProgramId: Id,
    BufferId: Id,
{
    program_id: ProgramId,
    buffer_id: BufferId,
    create_buffer_callback: CreateBufferCallback<UserCtx>,
    update_callback: BufferUpdateCallback<UserCtx>,
    should_update_callback: BufferShouldUpdateCallback<UserCtx>,
}

impl<ProgramId, BufferId, UserCtx> BufferLink<ProgramId, BufferId, UserCtx>
where
    ProgramId: Id,
    BufferId: Id,
{
    pub fn new(
        program_id: ProgramId,
        buffer_id: BufferId,
        create_buffer_callback: CreateBufferCallback<UserCtx>,
        update_callback: BufferUpdateCallback<UserCtx>,
        should_update_callback: BufferShouldUpdateCallback<UserCtx>,
    ) -> Self {
        Self {
            program_id,
            buffer_id,
            create_buffer_callback,
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

    pub fn create_buffer(
        &self,
        gl: &WebGl2RenderingContext,
        now: f64,
        attribute_location: &AttributeLocation,
        user_ctx: Option<&UserCtx>,
    ) -> WebGlBuffer {
        let create_buffer_context = CreateBufferContext::new(gl, now, attribute_location, user_ctx);
        (self.create_buffer_callback)(create_buffer_context)
    }

    pub fn update_callback(&self) -> BufferUpdateCallback<UserCtx> {
        Rc::clone(&self.update_callback)
    }

    pub fn should_update_callback(&self) -> BufferShouldUpdateCallback<UserCtx> {
        Rc::clone(&self.should_update_callback)
    }
}

impl<ProgramId, BufferId, UserCtx> Debug for BufferLink<ProgramId, BufferId, UserCtx>
where
    ProgramId: Id,
    BufferId: Id,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferLink")
            .field("program_id", &self.program_id)
            .field("buffer_id", &self.buffer_id)
            .field("update_callback", &"[not shown]")
            .field("should_update_callback", &"[not shown]")
            .finish()
    }
}

impl<ProgramId, BufferId, UserCtx> Hash for BufferLink<ProgramId, BufferId, UserCtx>
where
    ProgramId: Id,
    BufferId: Id,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.program_id.hash(state);
        self.buffer_id.hash(state);
    }
}

impl<ProgramId, BufferId, UserCtx> PartialEq for BufferLink<ProgramId, BufferId, UserCtx>
where
    ProgramId: Id,
    BufferId: Id,
{
    fn eq(&self, other: &Self) -> bool {
        self.program_id == other.program_id
            && self.buffer_id == other.buffer_id
            && Rc::ptr_eq(&self.update_callback, &other.update_callback)
            && Rc::ptr_eq(&self.should_update_callback, &other.should_update_callback)
    }
}

impl<ProgramId, BufferId, UserCtx> Eq for BufferLink<ProgramId, BufferId, UserCtx>
where
    ProgramId: Id,
    BufferId: Id,
{
}
