use std::fmt::Debug;
use std::hash::Hash;

use super::id::Id;

/// This contains an id for a program and and id for a uniform that is associated with it
/// At build time, these get linked together to find the uniform's associated location in the program
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct ProgramUniformLink<ProgramId, UniformId>
where
    ProgramId: Id,
    UniformId: Id,
{
    program_id: ProgramId,
    uniform_id: UniformId,
}

impl<ProgramId, UniformId> ProgramUniformLink<ProgramId, UniformId>
where
    ProgramId: Id,
    UniformId: Id,
{
    pub fn new(program_id: ProgramId, uniform_id: UniformId) -> Self {
        Self {
            program_id,
            uniform_id,
        }
    }

    pub fn program_id(&self) -> &ProgramId {
        &self.program_id
    }

    pub fn uniform_id(&self) -> &UniformId {
        &self.uniform_id
    }
}
