use crate::{
    AttributeContext, AttributeLocation, AttributeShouldUpdateCallback, AttributeUpdateCallback,
    Id, IdName,
};
use std::fmt::Debug;
use std::hash::Hash;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

#[derive(Clone)]
pub struct Attribute<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx: Clone> {
    program_id: ProgramId,
    buffer_id: BufferId,
    attribute_id: AttributeId,
    webgl_buffer: WebGlBuffer,
    attribute_location: AttributeLocation,
    update_callback: AttributeUpdateCallback<UserCtx>,
    should_update_callback: AttributeShouldUpdateCallback<UserCtx>,
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx: Clone>
    Attribute<ProgramId, BufferId, AttributeId, UserCtx>
{
    // @todo move into builder pattern
    pub fn new(
        program_id: ProgramId,
        buffer_id: BufferId,
        attribute_id: AttributeId,
        webgl_buffer: WebGlBuffer,
        attribute_location: AttributeLocation,
        update_callback: AttributeUpdateCallback<UserCtx>,
        should_update_callback: AttributeShouldUpdateCallback<UserCtx>,
    ) -> Self {
        Self {
            program_id,
            buffer_id,
            attribute_id,
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
        gl: WebGl2RenderingContext,
        now: f64,
        user_ctx: Option<UserCtx>,
    ) -> bool {
        let ctx = AttributeContext::new(
            gl,
            now,
            self.webgl_buffer().clone(),
            *self.attribute_location(),
            user_ctx,
        );
        (self.should_update_callback)(&ctx)
    }

    pub fn update(&self, gl: WebGl2RenderingContext, now: f64, user_ctx: Option<UserCtx>) {
        let ctx = AttributeContext::new(
            gl,
            now,
            self.webgl_buffer().clone(),
            *self.attribute_location(),
            user_ctx,
        );
        (self.update_callback)(&ctx);
    }
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx: Clone> Debug
    for Attribute<ProgramId, BufferId, AttributeId, UserCtx>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Buffer")
            .field("program_id", &self.program_id)
            .field("buffer_id", &self.buffer_id)
            .field("attribute_id", &self.attribute_id)
            .field("buffer", &self.webgl_buffer)
            .field("attribute_location", &self.attribute_location)
            .finish()
    }
}
impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx: Clone> Hash
    for Attribute<ProgramId, BufferId, AttributeId, UserCtx>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.buffer_id.hash(state);
        self.program_id.hash(state);
        self.attribute_id.hash(state);
    }
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx: Clone> PartialEq
    for Attribute<ProgramId, BufferId, AttributeId, UserCtx>
{
    fn eq(&self, other: &Self) -> bool {
        self.program_id == other.program_id
            && self.buffer_id == other.buffer_id
            && self.attribute_id == other.attribute_id
            && self.webgl_buffer == other.webgl_buffer
            && self.attribute_location == other.attribute_location
            && self.update_callback == other.update_callback
            && self.should_update_callback == other.should_update_callback
    }
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx: Clone> Eq
    for Attribute<ProgramId, BufferId, AttributeId, UserCtx>
{
}
