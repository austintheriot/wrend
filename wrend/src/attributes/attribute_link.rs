use crate::{
    AttributeCreateCallback, AttributeCreateContext, AttributeLocation, Id, Bridge, IdName,
};
use std::fmt::Debug;
use std::hash::Hash;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

#[derive(Clone)]
pub struct AttributeLink<
    VertexArrayObjectId: Id,
    BufferId: Id,
    AttributeId: Id + IdName,
    UserCtx: Clone,
> {
    vao_ids: Vec<VertexArrayObjectId>,
    buffer_id: BufferId,
    attribute_id: AttributeId,
    attribute_create_callback: AttributeCreateCallback<UserCtx>,
}

impl<VertexArrayObjectId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx: Clone>
    AttributeLink<VertexArrayObjectId, BufferId, AttributeId, UserCtx>
{
    pub fn new(
        vao_ids: impl Into<Bridge<VertexArrayObjectId>>,
        buffer_id: BufferId,
        attribute_id: AttributeId,
        attribute_create_callback: impl Into<AttributeCreateCallback<UserCtx>>,
    ) -> Self {
        let vao_ids_bridge = vao_ids.into();
        Self {
            vao_ids: vao_ids_bridge.into(),
            buffer_id,
            attribute_id,
            attribute_create_callback: attribute_create_callback.into(),
        }
    }

    pub fn vao_ids(&self) -> &[VertexArrayObjectId] {
        &self.vao_ids
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

impl<VertexArrayObjectId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx: Clone> Debug
    for AttributeLink<VertexArrayObjectId, BufferId, AttributeId, UserCtx>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AttributeLink")
            .field("vao_ids", &self.vao_ids)
            .field("buffer_id", &self.buffer_id)
            .field("attribute_id", &self.attribute_id)
            .field("update_callback", &"[not shown]")
            .field("should_update_callback", &"[not shown]")
            .finish()
    }
}

impl<VertexArrayObjectId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx: Clone> Hash
    for AttributeLink<VertexArrayObjectId, BufferId, AttributeId, UserCtx>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.vao_ids.hash(state);
        self.buffer_id.hash(state);
        self.attribute_id.hash(state);
    }
}

impl<VertexArrayObjectId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx: Clone> PartialEq
    for AttributeLink<VertexArrayObjectId, BufferId, AttributeId, UserCtx>
{
    fn eq(&self, other: &Self) -> bool {
        self.vao_ids == other.vao_ids
            && self.buffer_id == other.buffer_id
            && self.attribute_id == other.attribute_id
            && self.attribute_create_callback == other.attribute_create_callback
    }
}

impl<VertexArrayObjectId: Id, BufferId: Id, AttributeId: Id + IdName, UserCtx: Clone> Eq
    for AttributeLink<VertexArrayObjectId, BufferId, AttributeId, UserCtx>
{
}
