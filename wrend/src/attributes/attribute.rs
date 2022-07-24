use crate::{AttributeLocation, Id, Bridge, IdName};
use std::fmt::Debug;
use std::hash::Hash;
use web_sys::WebGlBuffer;

#[derive(Clone)]
pub struct Attribute<VertexArrayObjectId: Id, BufferId: Id, AttributeId: Id + IdName> {
    vao_ids: Vec<VertexArrayObjectId>,
    buffer_id: BufferId,
    attribute_id: AttributeId,
    webgl_buffer: WebGlBuffer,
    attribute_location: AttributeLocation,
}

impl<VertexArrayObjectId: Id, BufferId: Id, AttributeId: Id + IdName>
    Attribute<VertexArrayObjectId, BufferId, AttributeId>
{
    // @todo move into builder pattern
    pub fn new(
        vao_ids: impl Into<Bridge<VertexArrayObjectId>>,
        buffer_id: BufferId,
        attribute_id: AttributeId,
        webgl_buffer: WebGlBuffer,
        attribute_location: AttributeLocation,
    ) -> Self {
        let vao_id_bridge = vao_ids.into();
        Self {
            vao_ids: vao_id_bridge.into(),
            buffer_id,
            attribute_id,
            webgl_buffer,
            attribute_location,
        }
    }

    pub fn vao_ids(&self) -> &[VertexArrayObjectId] {
        &self.vao_ids
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
}

impl<VertexArrayObjectId: Id, BufferId: Id, AttributeId: Id + IdName> Debug
    for Attribute<VertexArrayObjectId, BufferId, AttributeId>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Buffer")
            .field("vao_ids", &self.vao_ids)
            .field("buffer_id", &self.buffer_id)
            .field("attribute_id", &self.attribute_id)
            .field("buffer", &self.webgl_buffer)
            .field("attribute_location", &self.attribute_location)
            .finish()
    }
}
impl<VertexArrayObjectId: Id, BufferId: Id, AttributeId: Id + IdName> Hash
    for Attribute<VertexArrayObjectId, BufferId, AttributeId>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.buffer_id.hash(state);
        self.vao_ids.hash(state);
        self.attribute_id.hash(state);
    }
}

impl<VertexArrayObjectId: Id, BufferId: Id, AttributeId: Id + IdName> PartialEq
    for Attribute<VertexArrayObjectId, BufferId, AttributeId>
{
    fn eq(&self, other: &Self) -> bool {
        self.vao_ids == other.vao_ids
            && self.buffer_id == other.buffer_id
            && self.attribute_id == other.attribute_id
            && self.webgl_buffer == other.webgl_buffer
            && self.attribute_location == other.attribute_location
    }
}

impl<VertexArrayObjectId: Id, BufferId: Id, AttributeId: Id + IdName> Eq
    for Attribute<VertexArrayObjectId, BufferId, AttributeId>
{
}
