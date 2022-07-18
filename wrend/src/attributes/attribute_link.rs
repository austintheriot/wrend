use crate::{AttributeCreateCallback, AttributeCreateContext, AttributeLocation, Id, IdName};
use std::fmt::Debug;
use std::hash::Hash;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

#[derive(Clone)]
pub struct AttributeLink<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx: Clone> {
    program_id: ProgramId,
    buffer_id: BufferId,
    attribute_id: AttributeId,
    attribute_create_callback: AttributeCreateCallback<UserCtx>,
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx: Clone>
    AttributeLink<ProgramId, BufferId, AttributeId, UserCtx>
{
    pub fn new(
        program_id: ProgramId,
        buffer_id: BufferId,
        attribute_id: AttributeId,
        attribute_create_callback: impl Into<AttributeCreateCallback<UserCtx>>,
    ) -> Self {
        Self {
            program_id,
            buffer_id,
            attribute_id,
            attribute_create_callback: attribute_create_callback.into(),
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
        gl: WebGl2RenderingContext,
        now: f64,
        webgl_buffer: WebGlBuffer,
        attribute_location: AttributeLocation,
        user_ctx: Option<UserCtx>,
    ) {
        let attribute_create_context =
            AttributeCreateContext::new(gl, now, webgl_buffer, attribute_location, user_ctx);
        (self.attribute_create_callback)(&attribute_create_context)
    }

    pub fn create_callback(&self) -> AttributeCreateCallback<UserCtx> {
        self.attribute_create_callback.clone()
    }
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx: Clone> Debug
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

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx: Clone> Hash
    for AttributeLink<ProgramId, BufferId, AttributeId, UserCtx>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.program_id.hash(state);
        self.buffer_id.hash(state);
        self.attribute_id.hash(state);
    }
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx: Clone> PartialEq
    for AttributeLink<ProgramId, BufferId, AttributeId, UserCtx>
{
    fn eq(&self, other: &Self) -> bool {
        self.program_id == other.program_id
            && self.buffer_id == other.buffer_id
            && self.program_id == other.program_id
            && self.attribute_id == other.attribute_id
    }
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx: Clone> Eq
    for AttributeLink<ProgramId, BufferId, AttributeId, UserCtx>
{
}
