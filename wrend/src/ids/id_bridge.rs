use crate::Id;

/// Enables accepting either a single Id or many Ids when taking function arguments
pub struct IdBridge<ProgramId: Id>(Vec<ProgramId>);

impl<ProgramId: Id> From<IdBridge<ProgramId>> for Vec<ProgramId> {
    fn from(program_id_bridge: IdBridge<ProgramId>) -> Self {
        program_id_bridge.0
    }
}

impl<ProgramId: Id> From<&IdBridge<ProgramId>> for Vec<ProgramId> {
    fn from(program_id_bridge: &IdBridge<ProgramId>) -> Self {
        program_id_bridge.0.to_owned()
    }
}

impl<ProgramId: Id> From<(ProgramId, ProgramId)> for IdBridge<ProgramId> {
    fn from(program_id: (ProgramId, ProgramId)) -> Self {
        IdBridge(vec![program_id.0, program_id.1])
    }
}

impl<ProgramId: Id> From<(ProgramId, ProgramId, ProgramId)> for IdBridge<ProgramId> {
    fn from(program_id: (ProgramId, ProgramId, ProgramId)) -> Self {
        IdBridge(vec![program_id.0, program_id.1, program_id.2])
    }
}

impl<ProgramId: Id> From<(ProgramId, ProgramId, ProgramId, ProgramId)> for IdBridge<ProgramId> {
    fn from(program_id: (ProgramId, ProgramId, ProgramId, ProgramId)) -> Self {
        IdBridge(vec![program_id.0, program_id.1, program_id.2, program_id.3])
    }
}

impl<ProgramId: Id> From<ProgramId> for IdBridge<ProgramId> {
    fn from(program_id: ProgramId) -> Self {
        IdBridge(vec![program_id])
    }
}

impl<ProgramId: Id> From<&ProgramId> for IdBridge<ProgramId> {
    fn from(program_id: &ProgramId) -> Self {
        IdBridge(vec![program_id.to_owned()])
    }
}

impl<ProgramId: Id> From<Vec<ProgramId>> for IdBridge<ProgramId> {
    fn from(program_ids: Vec<ProgramId>) -> Self {
        IdBridge(program_ids)
    }
}

impl<ProgramId: Id> From<&Vec<ProgramId>> for IdBridge<ProgramId> {
    fn from(program_ids: &Vec<ProgramId>) -> Self {
        IdBridge(program_ids.to_owned())
    }
}

impl<ProgramId: Id> From<&[ProgramId]> for IdBridge<ProgramId> {
    fn from(program_ids: &[ProgramId]) -> Self {
        IdBridge(program_ids.to_vec())
    }
}
