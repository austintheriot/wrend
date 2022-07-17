use crate::{
    AttributeCreateContext, AttributeLocation, AttributeShouldUpdateCallback,
    AttributeUpdateCallback, Id, IdName,
};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

pub type AttributeCreateCallback<UserCtx> = Rc<dyn Fn(AttributeCreateContext<UserCtx>)>;

#[derive(Clone)]
pub struct AttributeLink<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx> {
    program_id: ProgramId,
    buffer_id: BufferId,
    attribute_id: AttributeId,
    attribute_create_callback: AttributeCreateCallback<UserCtx>,
    update_callback: AttributeUpdateCallback<UserCtx>,
    should_update_callback: AttributeShouldUpdateCallback<UserCtx>,
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx>
    AttributeLink<ProgramId, BufferId, AttributeId, UserCtx>
{
    pub fn new(
        program_id: ProgramId,
        buffer_id: BufferId,
        attribute_id: AttributeId,
        attribute_create_callback: AttributeCreateCallback<UserCtx>,
        update_callback: AttributeUpdateCallback<UserCtx>,
        should_update_callback: AttributeShouldUpdateCallback<UserCtx>,
    ) -> Self {
        Self {
            program_id,
            buffer_id,
            attribute_id,
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

    pub fn attribute_id(&self) -> &AttributeId {
        &self.attribute_id
    }

    pub fn configure_attribute(
        &self,
        gl: &WebGl2RenderingContext,
        now: f64,
        webgl_buffer: &WebGlBuffer,
        attribute_location: &AttributeLocation,
        user_ctx: Option<&UserCtx>,
    ) {
        let attribute_create_context =
            AttributeCreateContext::new(gl, now, webgl_buffer, attribute_location, user_ctx);
        (self.attribute_create_callback)(attribute_create_context)
    }

    pub fn create_callback(&self) -> AttributeCreateCallback<UserCtx> {
        Rc::clone(&self.attribute_create_callback)
    }

    pub fn update_callback(&self) -> AttributeUpdateCallback<UserCtx> {
        Rc::clone(&self.update_callback)
    }

    pub fn should_update_callback(&self) -> AttributeShouldUpdateCallback<UserCtx> {
        Rc::clone(&self.should_update_callback)
    }
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx> Debug
    for AttributeLink<ProgramId, BufferId, AttributeId, UserCtx>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AttributeLink")
            .field("program_id", &self.program_id)
            .field("buffer_id", &self.buffer_id)
            .field("attribute_id", &self.attribute_id)
            .field("update_callback", &"[not shown]")
            .field("should_update_callback", &"[not shown]")
            .finish()
    }
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx> Hash
    for AttributeLink<ProgramId, BufferId, AttributeId, UserCtx>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.program_id.hash(state);
        self.buffer_id.hash(state);
        self.attribute_id.hash(state);
    }
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx> PartialEq
    for AttributeLink<ProgramId, BufferId, AttributeId, UserCtx>
{
    fn eq(&self, other: &Self) -> bool {
        self.program_id == other.program_id
            && self.buffer_id == other.buffer_id
            && self.program_id == other.program_id
            && self.attribute_id == other.attribute_id
    }
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx> Eq
    for AttributeLink<ProgramId, BufferId, AttributeId, UserCtx>
{
}
