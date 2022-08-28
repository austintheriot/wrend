use crate::Bridge;
use crate::Id;
use crate::UniformCreateUpdateCallback;
use crate::UniformShouldUpdateCallback;
use std::fmt::Debug;
use std::hash::Hash;

/// This contains one or more ids for a program and and id for a uniform that is associated with it
/// At build time, these get linked together to find the uniform's associated location in the program
#[derive(Clone)]
pub struct UniformLink<ProgramId: Id, UniformId: Id, UserCtx: Clone> {
    program_ids: Vec<ProgramId>,
    uniform_id: UniformId,
    initialize_callback: UniformCreateUpdateCallback<UserCtx>,
    update_callback: Option<UniformCreateUpdateCallback<UserCtx>>,
    should_update_callback: Option<UniformShouldUpdateCallback<UserCtx>>,
    use_init_callback_for_update: bool,
}

impl<ProgramId: Id, UniformId: Id, UserCtx: Clone> UniformLink<ProgramId, UniformId, UserCtx> {
    pub fn new(
        program_ids: impl Into<Bridge<ProgramId>>,
        uniform_id: UniformId,
        initialize_callback: impl Into<UniformCreateUpdateCallback<UserCtx>>,
    ) -> Self {
        let program_id_bridge: Bridge<ProgramId> = program_ids.into();
        let program_ids = program_id_bridge.into();
        Self {
            program_ids,
            uniform_id,
            initialize_callback: initialize_callback.into(),
            use_init_callback_for_update: false,
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

    pub fn initialize_callback(&self) -> UniformCreateUpdateCallback<UserCtx> {
        self.initialize_callback.clone()
    }

    pub fn set_initialize_callback(
        &mut self,
        callback: impl Into<UniformCreateUpdateCallback<UserCtx>>,
    ) -> &mut Self {
        self.initialize_callback = callback.into();
        self
    }

    pub fn should_update_callback(&self) -> Option<UniformShouldUpdateCallback<UserCtx>> {
        self.should_update_callback.as_ref().map(Clone::clone)
    }

    pub fn set_should_update_callback(
        &mut self,
        callback: impl Into<UniformShouldUpdateCallback<UserCtx>>,
    ) -> &mut Self {
        self.should_update_callback.replace(callback.into());
        self
    }

    pub fn update_callback(&self) -> Option<UniformCreateUpdateCallback<UserCtx>> {
        self.update_callback.as_ref().map(Clone::clone)
    }

    pub fn set_update_callback(
        &mut self,
        callback: impl Into<UniformCreateUpdateCallback<UserCtx>>,
    ) -> &mut Self {
        self.update_callback.replace(callback.into());
        self
    }

    pub fn use_init_callback_for_update(&self) -> bool {
        self.use_init_callback_for_update
    }

    pub fn set_use_init_callback_for_update(
        &mut self,
        use_init_callback_for_update: bool,
    ) -> &mut Self {
        self.use_init_callback_for_update = use_init_callback_for_update;
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
            .field(
                "use_init_callback_for_update",
                &self.use_init_callback_for_update,
            )
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
