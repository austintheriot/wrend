use crate::Id;
use crate::IdBridge;
use crate::UniformCallback;
use crate::UniformShouldUpdateCallback;
use std::fmt::Debug;
use std::hash::Hash;

/// This contains one or more ids for a program and and id for a uniform that is associated with it
/// At build time, these get linked together to find the uniform's associated location in the program
#[derive(Clone)]
pub struct UniformLink<ProgramId: Id, UniformId: Id, UserCtx: Clone> {
    program_ids: Vec<ProgramId>,
    uniform_id: UniformId,
    initialize_callback: UniformCallback<UserCtx>,
    update_callback: Option<UniformCallback<UserCtx>>,
    should_update_callback: Option<UniformShouldUpdateCallback<UserCtx>>,
}

impl<ProgramId: Id, UniformId: Id, UserCtx: Clone> UniformLink<ProgramId, UniformId, UserCtx> {
    pub fn new(
        program_ids: impl Into<IdBridge<ProgramId>>,
        uniform_id: UniformId,
        initialize_callback: UniformCallback<UserCtx>,
    ) -> Self {
        let program_id_bridge: IdBridge<ProgramId> = program_ids.into();
        let program_ids = program_id_bridge.into();
        Self {
            program_ids,
            uniform_id,
            initialize_callback,
            should_update_callback: None,
            update_callback: None,
        }
    }

    pub fn program_ids(&self) -> &Vec<ProgramId> {
        &self.program_ids
    }

    pub fn uniform_id(&self) -> &UniformId {
        &self.uniform_id
    }

    pub fn initialize_callback(&self) -> UniformCallback<UserCtx> {
        self.initialize_callback.clone()
    }

    pub fn set_initialize_callback(&mut self, callback: UniformCallback<UserCtx>) -> &mut Self {
        self.initialize_callback = callback;
        self
    }

    pub fn should_update_callback(&self) -> Option<UniformShouldUpdateCallback<UserCtx>> {
        self.should_update_callback.as_ref().map(Clone::clone)
    }

    pub fn set_should_update_callback(
        &mut self,
        callback: UniformShouldUpdateCallback<UserCtx>,
    ) -> &mut Self {
        self.should_update_callback.replace(callback);
        self
    }

    pub fn update_callback(&self) -> Option<UniformCallback<UserCtx>> {
        self.update_callback.as_ref().map(Clone::clone)
    }

    pub fn set_update_callback(&mut self, callback: UniformCallback<UserCtx>) -> &mut Self {
        self.update_callback.replace(callback);
        self
    }
}

impl<ProgramId: Id, UniformId: Id, UserCtx: Clone> Debug
    for UniformLink<ProgramId, UniformId, UserCtx>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UniformLink")
            .field("program_ids", &self.program_ids)
            .field("uniform_id", &self.uniform_id)
            .field("initialize_callback", &self.initialize_callback)
            .field("update_callback", &self.update_callback)
            .field("should_update_callback", &self.should_update_callback)
            .finish()
    }
}

impl<ProgramId: Id, UniformId: Id, UserCtx: Clone> Hash
    for UniformLink<ProgramId, UniformId, UserCtx>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.program_ids.hash(state);
        self.uniform_id.hash(state);
    }
}

impl<ProgramId: Id, UniformId: Id, UserCtx: Clone> PartialEq
    for UniformLink<ProgramId, UniformId, UserCtx>
{
    fn eq(&self, other: &Self) -> bool {
        self.program_ids == other.program_ids && self.uniform_id == other.uniform_id
    }
}

impl<ProgramId: Id, UniformId: Id, UserCtx: Clone> Eq
    for UniformLink<ProgramId, UniformId, UserCtx>
{
}
