use crate::{AttributeLocation, Id, IdName};
use std::fmt::Debug;
use std::hash::Hash;
use web_sys::WebGlBuffer;

#[derive(Clone)]
pub struct Attribute<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName> {
    program_id: ProgramId,
    buffer_id: BufferId,
    attribute_id: AttributeId,
    webgl_buffer: WebGlBuffer,
    attribute_location: AttributeLocation,
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName>
    Attribute<ProgramId, BufferId, AttributeId>
{
    // @todo move into builder pattern
    pub fn new(
        program_id: ProgramId,
        buffer_id: BufferId,
        attribute_id: AttributeId,
        webgl_buffer: WebGlBuffer,
        attribute_location: AttributeLocation,
    ) -> Self {
        Self {
            program_id,
            buffer_id,
            attribute_id,
            webgl_buffer,
            attribute_location,
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
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName> Debug
    for Attribute<ProgramId, BufferId, AttributeId>
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
impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName> Hash
    for Attribute<ProgramId, BufferId, AttributeId>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.buffer_id.hash(state);
        self.program_id.hash(state);
        self.attribute_id.hash(state);
    }
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName> PartialEq
    for Attribute<ProgramId, BufferId, AttributeId>
{
    fn eq(&self, other: &Self) -> bool {
        self.program_id == other.program_id
            && self.buffer_id == other.buffer_id
            && self.attribute_id == other.attribute_id
            && self.webgl_buffer == other.webgl_buffer
            && self.attribute_location == other.attribute_location
    }
}

impl<ProgramId: Id, BufferId: Id, AttributeId: Id + IdName> Eq
    for Attribute<ProgramId, BufferId, AttributeId>
{
}
