use crate::Id;

/// Enables accepting either a single ProgramId or many ProgramIds when creating a uniform
pub struct ProgramIdBridge<ProgramId: Id>(Vec<ProgramId>);

impl<ProgramId: Id> From<ProgramIdBridge<ProgramId>> for Vec<ProgramId> {
    fn from(program_id_bridge: ProgramIdBridge<ProgramId>) -> Self {
        program_id_bridge.0
    }
}

impl<ProgramId: Id> From<&ProgramIdBridge<ProgramId>> for Vec<ProgramId> {
    fn from(program_id_bridge: &ProgramIdBridge<ProgramId>) -> Self {
        program_id_bridge.0.to_owned()
    }
}

impl<ProgramId: Id> From<(ProgramId, ProgramId)> for ProgramIdBridge<ProgramId> {
    fn from(program_id: (ProgramId, ProgramId)) -> Self {
        ProgramIdBridge(vec![program_id.0, program_id.1])
    }
}

impl<ProgramId: Id> From<(ProgramId, ProgramId, ProgramId)> for ProgramIdBridge<ProgramId> {
    fn from(program_id: (ProgramId, ProgramId, ProgramId)) -> Self {
        ProgramIdBridge(vec![program_id.0, program_id.1, program_id.2])
    }
}

impl<ProgramId: Id> From<(ProgramId, ProgramId, ProgramId, ProgramId)>
    for ProgramIdBridge<ProgramId>
{
    fn from(program_id: (ProgramId, ProgramId, ProgramId, ProgramId)) -> Self {
        ProgramIdBridge(vec![program_id.0, program_id.1, program_id.2, program_id.3])
    }
}

impl<ProgramId: Id> From<ProgramId> for ProgramIdBridge<ProgramId> {
    fn from(program_id: ProgramId) -> Self {
        ProgramIdBridge(vec![program_id])
    }
}

impl<ProgramId: Id> From<&ProgramId> for ProgramIdBridge<ProgramId> {
    fn from(program_id: &ProgramId) -> Self {
        ProgramIdBridge(vec![program_id.to_owned()])
    }
}

impl<ProgramId: Id> From<Vec<ProgramId>> for ProgramIdBridge<ProgramId> {
    fn from(program_ids: Vec<ProgramId>) -> Self {
        ProgramIdBridge(program_ids)
    }
}

impl<ProgramId: Id> From<&Vec<ProgramId>> for ProgramIdBridge<ProgramId> {
    fn from(program_ids: &Vec<ProgramId>) -> Self {
        ProgramIdBridge(program_ids.to_owned())
    }
}

impl<ProgramId: Id> From<&[ProgramId]> for ProgramIdBridge<ProgramId> {
    fn from(program_ids: &[ProgramId]) -> Self {
        ProgramIdBridge(program_ids.to_vec())
    }
}
