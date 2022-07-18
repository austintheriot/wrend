use crate::{AttributeLocation, Id, IdName, IdBridge};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use web_sys::WebGlBuffer;

#[derive(Clone)]
pub struct Attribute<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName> {
    program_ids: Vec<ProgramId>,
    buffer_id: BufferId,
    attribute_id: AttributeId,
    webgl_buffer: WebGlBuffer,
    attribute_locations: HashMap<ProgramId, AttributeLocation>,
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName>
    Attribute<ProgramId, BufferId, AttributeId>
{
    // @todo move into builder pattern
    pub fn new(
        program_ids: impl Into<IdBridge<ProgramId>>,
        buffer_id: BufferId,
        attribute_id: AttributeId,
        webgl_buffer: WebGlBuffer,
        attribute_locations: HashMap<ProgramId, AttributeLocation>,
    ) -> Self {
        let program_id_bridge = program_ids.into();
        Self {
            program_ids: program_id_bridge.into(),
            buffer_id,
            attribute_id,
            webgl_buffer,
            attribute_locations,
        }
    }

    pub fn program_ids(&self) -> &[ProgramId] {
        &self.program_ids
    }

    pub fn buffer_id(&self) -> &BufferId {
        &self.buffer_id
    }

    pub fn webgl_buffer(&self) -> &WebGlBuffer {
        &self.webgl_buffer
    }

    pub fn attribute_locations(&self) -> &HashMap<ProgramId, AttributeLocation> {
        &self.attribute_locations
    }
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName> Debug
    for Attribute<ProgramId, BufferId, AttributeId>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Buffer")
            .field("program_ids", &self.program_ids)
            .field("buffer_id", &self.buffer_id)
            .field("attribute_id", &self.attribute_id)
            .field("buffer", &self.webgl_buffer)
            .field("attribute_locations", &self.attribute_locations)
            .finish()
    }
}
impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName> Hash
    for Attribute<ProgramId, BufferId, AttributeId>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.buffer_id.hash(state);
        self.program_ids.hash(state);
        self.attribute_id.hash(state);
    }
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName> PartialEq
    for Attribute<ProgramId, BufferId, AttributeId>
{
    fn eq(&self, other: &Self) -> bool {
        self.program_ids == other.program_ids
            && self.buffer_id == other.buffer_id
            && self.attribute_id == other.attribute_id
            && self.webgl_buffer == other.webgl_buffer
            && self.attribute_locations == other.attribute_locations
    }
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName> Eq
    for Attribute<ProgramId, BufferId, AttributeId>
{
}
