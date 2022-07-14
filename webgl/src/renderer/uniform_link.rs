use super::id::Id;
use super::uniform::UniformUpdateCallback;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

/// This contains an id for a program and and id for a uniform that is associated with it
/// At build time, these get linked together to find the uniform's associated location in the program
#[derive(Clone)]
pub struct UniformLink<ProgramId, UniformId, UserCtx>
where
    ProgramId: Id,
    UniformId: Id,
{
    program_id: ProgramId,
    uniform_id: UniformId,
    update_callback: UniformUpdateCallback<UserCtx>,
}

impl<ProgramId, UniformId, UserCtx> UniformLink<ProgramId, UniformId, UserCtx>
where
    ProgramId: Id,
    UniformId: Id,
{
    pub fn new(
        program_id: ProgramId,
        uniform_id: UniformId,
        update_callback: UniformUpdateCallback<UserCtx>,
    ) -> Self {
        Self {
            program_id,
            uniform_id,
            update_callback,
        }
    }

    pub fn program_id(&self) -> &ProgramId {
        &self.program_id
    }

    pub fn uniform_id(&self) -> &UniformId {
        &self.uniform_id
    }

    pub fn update_callback(&self) -> UniformUpdateCallback<UserCtx> {
        Rc::clone(&self.update_callback)
    }
}

impl<ProgramId, UniformId, UserCtx> Debug for UniformLink<ProgramId, UniformId, UserCtx>
where
    ProgramId: Id,
    UniformId: Id,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UniformLink")
            .field("program_id", &self.program_id)
            .field("uniform_id", &self.uniform_id)
            .field("update_callback", &"[not shown]")
            .finish()
    }
}

impl<ProgramId, UniformId, UserCtx> Hash for UniformLink<ProgramId, UniformId, UserCtx>
where
    ProgramId: Id,
    UniformId: Id,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.program_id.hash(state);
        self.uniform_id.hash(state);
    }
}

impl<ProgramId, UniformId, UserCtx> PartialEq for UniformLink<ProgramId, UniformId, UserCtx>
where
    ProgramId: Id,
    UniformId: Id,
{
    fn eq(&self, other: &Self) -> bool {
        self.program_id == other.program_id
            && self.uniform_id == other.uniform_id
            && Rc::ptr_eq(&self.update_callback, &other.update_callback)
    }
}

impl<ProgramId, UniformId, UserCtx> Eq for UniformLink<ProgramId, UniformId, UserCtx>
where
    ProgramId: Id,
    UniformId: Id,
{
}
