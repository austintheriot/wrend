use crate::{
    AttributeCreateCallback, AttributeCreateContext, AttributeLinkJs, AttributeLinkJsInner,
    AttributeLocation, Bridge, Id, IdName,
};
use std::fmt::Debug;
use std::hash::Hash;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

#[derive(Clone)]
pub struct AttributeLink<VertexArrayObjectId: Id, BufferId: Id, AttributeId: Id + IdName> {
    vao_ids: Vec<VertexArrayObjectId>,
    buffer_id: BufferId,
    attribute_id: AttributeId,
    attribute_create_callback: AttributeCreateCallback,
}

impl<VertexArrayObjectId: Id, BufferId: Id, AttributeId: Id + IdName>
    AttributeLink<VertexArrayObjectId, BufferId, AttributeId>
{
    pub fn new(
        vao_ids: impl Into<Bridge<VertexArrayObjectId>>,
        buffer_id: BufferId,
        attribute_id: AttributeId,
        attribute_create_callback: impl Into<AttributeCreateCallback>,
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

    pub fn create_callback(&self) -> AttributeCreateCallback {
        self.attribute_create_callback.clone()
    }

    /// Runs the associated `attribute_create_callback` to initialize the attribute
    pub fn create_attribute(
        &self,
        gl: WebGl2RenderingContext,
        now: f64,
        webgl_buffer: WebGlBuffer,
        attribute_location: AttributeLocation,
    ) {
        let attribute_create_context =
            AttributeCreateContext::new(gl, now, webgl_buffer, attribute_location);
        self.attribute_create_callback
            .call_with_arg_into_js_value(&attribute_create_context);
    }
}

impl<VertexArrayObjectId: Id, BufferId: Id, AttributeId: Id + IdName> Debug
    for AttributeLink<VertexArrayObjectId, BufferId, AttributeId>
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

impl<VertexArrayObjectId: Id, BufferId: Id, AttributeId: Id + IdName> Hash
    for AttributeLink<VertexArrayObjectId, BufferId, AttributeId>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.vao_ids.hash(state);
        self.buffer_id.hash(state);
        self.attribute_id.hash(state);
    }
}

impl<VertexArrayObjectId: Id, BufferId: Id, AttributeId: Id + IdName> PartialEq
    for AttributeLink<VertexArrayObjectId, BufferId, AttributeId>
{
    fn eq(&self, other: &Self) -> bool {
        self.vao_ids == other.vao_ids
            && self.buffer_id == other.buffer_id
            && self.attribute_id == other.attribute_id
            && self.attribute_create_callback == other.attribute_create_callback
    }
}

impl<VertexArrayObjectId: Id, BufferId: Id, AttributeId: Id + IdName> Eq
    for AttributeLink<VertexArrayObjectId, BufferId, AttributeId>
{
}

impl From<AttributeLinkJs> for AttributeLinkJsInner {
    fn from(js_attribute_link: AttributeLinkJs) -> Self {
        js_attribute_link.inner()
    }
}
