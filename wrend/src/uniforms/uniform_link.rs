use crate::Bridge;
use crate::Id;
use crate::UniformCreateUpdateCallback;
use crate::UniformShouldUpdateCallback;
use std::fmt::Debug;
use std::hash::Hash;

/// This link is used to associated a one or more `ProgramId`s with a conceptual uniform.
/// This allows sharing uniforms across programs, even when each uniform's location differs between programs.
#[derive(Clone)]
pub struct UniformLink<ProgramId: Id, UniformId: Id> {
    program_ids: Vec<ProgramId>,
    uniform_id: UniformId,
    initialize_callback: UniformCreateUpdateCallback,
    update_callback: Option<UniformCreateUpdateCallback>,
    should_update_callback: Option<UniformShouldUpdateCallback>,
    use_init_callback_for_update: bool,
}

impl<ProgramId: Id, UniformId: Id> UniformLink<ProgramId, UniformId> {
    /// Creates a new uniform link
    pub fn new(
        program_ids: impl Into<Bridge<ProgramId>>,
        uniform_id: UniformId,
        initialize_callback: impl Into<UniformCreateUpdateCallback>,
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

    /// Gets all program ids that this link is associated with
    pub fn program_ids(&self) -> &Vec<ProgramId> {
        &self.program_ids
    }

    /// Gets the id of the uniform link
    pub fn uniform_id(&self) -> &UniformId {
        &self.uniform_id
    }

    /// See [crate::Uniform::initialize_callback]
    pub fn initialize_callback(&self) -> UniformCreateUpdateCallback {
        self.initialize_callback.clone()
    }

    /// See [crate::Uniform::initialize_callback]
    pub fn set_initialize_callback(
        &mut self,
        callback: impl Into<UniformCreateUpdateCallback>,
    ) -> &mut Self {
        self.initialize_callback = callback.into();
        self
    }

    /// See [crate::Uniform::should_update_callback]
    pub fn should_update_callback(&self) -> Option<UniformShouldUpdateCallback> {
        self.should_update_callback.as_ref().map(Clone::clone)
    }

    /// See [crate::Uniform::should_update_callback]
    pub fn set_should_update_callback(
        &mut self,
        callback: impl Into<UniformShouldUpdateCallback>,
    ) -> &mut Self {
        self.should_update_callback.replace(callback.into());
        self
    }

    /// See [crate::Uniform::update_callback]
    pub fn update_callback(&self) -> Option<UniformCreateUpdateCallback> {
        self.update_callback.as_ref().map(Clone::clone)
    }

    /// See [crate::Uniform::update_callback]
    pub fn set_update_callback(
        &mut self,
        callback: impl Into<UniformCreateUpdateCallback>,
    ) -> &mut Self {
        self.update_callback.replace(callback.into());
        self
    }

    /// See [Uniform::use_init_callback_for_update]
    pub fn use_init_callback_for_update(&self) -> bool {
        self.use_init_callback_for_update
    }

    /// See [Uniform::use_init_callback_for_update]
    pub fn set_use_init_callback_for_update(
        &mut self,
        use_init_callback_for_update: bool,
    ) -> &mut Self {
        self.use_init_callback_for_update = use_init_callback_for_update;
        self
    }
}

impl<ProgramId: Id, UniformId: Id> Debug for UniformLink<ProgramId, UniformId> {
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

impl<ProgramId: Id, UniformId: Id> Hash for UniformLink<ProgramId, UniformId> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.program_ids.hash(state);
        self.uniform_id.hash(state);
    }
}

impl<ProgramId: Id, UniformId: Id> PartialEq for UniformLink<ProgramId, UniformId> {
    fn eq(&self, other: &Self) -> bool {
        self.program_ids == other.program_ids && self.uniform_id == other.uniform_id
    }
}

impl<ProgramId: Id, UniformId: Id> Eq for UniformLink<ProgramId, UniformId> {}
