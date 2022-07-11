use super::id::Id;
use std::fmt::Debug;
use std::hash::Hash;

/// This contains an id for a pair of shaders: one vertex shader and one fragment
/// shader. These can be combined to link together a program.
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct ProgramLink<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id> {
    program_id: ProgramId,
    vertex_shader_id: VertexShaderId,
    fragment_shader_id: FragmentShaderId,
}

impl<ProgramId: Id, VertexShaderId: Id, FragmentShaderId: Id>
    ProgramLink<ProgramId, VertexShaderId, FragmentShaderId>
{
    pub fn new(
        program_id: ProgramId,
        vertex_shader_id: VertexShaderId,
        fragment_shader_id: FragmentShaderId,
    ) -> Self {
        Self {
            program_id,
            vertex_shader_id,
            fragment_shader_id,
        }
    }

    pub fn program_id(&self) -> &ProgramId {
        &self.program_id
    }

    pub fn vertex_shader_id(&self) -> &VertexShaderId {
        &self.vertex_shader_id
    }

    pub fn fragment_shader_id(&self) -> &FragmentShaderId {
        &self.fragment_shader_id
    }
}
