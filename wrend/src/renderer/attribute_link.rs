use super::attribute_location::AttributeLocation;
use super::attribute::{AttributeShouldUpdateCallback, AttributeUpdateCallback};
use super::attribute_create_context::AttributeCreateContext;
use super::id::Id;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

pub type AttributeCreateCallback<UserCtx> = Rc<dyn Fn(AttributeCreateContext<UserCtx>) -> WebGlBuffer>;

#[derive(Clone)]
pub struct AttributeLink<ProgramId, BufferId, UserCtx>
where
    ProgramId: Id,
    BufferId: Id,
{
    program_id: ProgramId,
    buffer_id: BufferId,
    attribute_create_callback: AttributeCreateCallback<UserCtx>,
    update_callback: AttributeUpdateCallback<UserCtx>,
    should_update_callback: AttributeShouldUpdateCallback<UserCtx>,
}

impl<ProgramId, BufferId, UserCtx> AttributeLink<ProgramId, BufferId, UserCtx>
where
    ProgramId: Id,
    BufferId: Id,
{
    pub fn new(
        program_id: ProgramId,
        buffer_id: BufferId,
        attribute_create_callback: AttributeCreateCallback<UserCtx>,
        update_callback: AttributeUpdateCallback<UserCtx>,
        should_update_callback: AttributeShouldUpdateCallback<UserCtx>,
    ) -> Self {
        Self {
            program_id,
            buffer_id,
            attribute_create_callback,
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
        let attribute_create_context = AttributeCreateContext::new(gl, now, attribute_location, user_ctx);
        (self.attribute_create_callback)(attribute_create_context)
    }

    pub fn update_callback(&self) -> AttributeUpdateCallback<UserCtx> {
        Rc::clone(&self.update_callback)
    }

    pub fn should_update_callback(&self) -> AttributeShouldUpdateCallback<UserCtx> {
        Rc::clone(&self.should_update_callback)
    }
}

impl<ProgramId, BufferId, UserCtx> Debug for AttributeLink<ProgramId, BufferId, UserCtx>
where
    ProgramId: Id,
    BufferId: Id,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AttributeLink")
            .field("program_id", &self.program_id)
            .field("buffer_id", &self.buffer_id)
            .field("update_callback", &"[not shown]")
            .field("should_update_callback", &"[not shown]")
            .finish()
    }
}

impl<ProgramId, BufferId, UserCtx> Hash for AttributeLink<ProgramId, BufferId, UserCtx>
where
    ProgramId: Id,
    BufferId: Id,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.program_id.hash(state);
        self.buffer_id.hash(state);
    }
}

impl<ProgramId, BufferId, UserCtx> PartialEq for AttributeLink<ProgramId, BufferId, UserCtx>
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

impl<ProgramId, BufferId, UserCtx> Eq for AttributeLink<ProgramId, BufferId, UserCtx>
where
    ProgramId: Id,
    BufferId: Id,
{
}
