use super::uniform_callback::UniformCallback;
use super::uniform_should_update_callback::UniformShouldUpdateCallback;
use super::{id::Id, uniform_context::UniformContext};
use std::fmt::Debug;
use std::hash::Hash;
use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

#[derive(Clone)]
pub struct Uniform<ProgramId: Id, UniformId: Id, UserCtx> {
    program_id: ProgramId,
    uniform_id: UniformId,
    uniform_location: WebGlUniformLocation,
    initialize_callback: UniformCallback<UserCtx>,
    update_callback: Option<UniformCallback<UserCtx>>,
    should_update_callback: Option<UniformShouldUpdateCallback<UserCtx>>,
}

impl<ProgramId: Id, UniformId: Id, UserCtx> Uniform<ProgramId, UniformId, UserCtx> {
    // @todo move into builder pattern
    pub fn new(
        program_id: ProgramId,
        uniform_id: UniformId,
        uniform_location: WebGlUniformLocation,
        initialize_callback: UniformCallback<UserCtx>,
        update_callback: Option<UniformCallback<UserCtx>>,
        should_update_callback: Option<UniformShouldUpdateCallback<UserCtx>>,
    ) -> Self {
        Self {
            program_id,
            uniform_id,
            uniform_location,
            initialize_callback,
            update_callback,
            should_update_callback,
        }
    }

    pub fn program_id(&self) -> &ProgramId {
        &self.program_id
    }

    pub fn uniform_id(&self) -> &UniformId {
        &self.uniform_id
    }

    pub fn uniform_location(&self) -> &WebGlUniformLocation {
        &self.uniform_location
    }

    pub fn initialize_callback(&self) -> UniformCallback<UserCtx> {
        self.initialize_callback.clone()
    }

    pub fn should_update_callback(&self) -> Option<UniformShouldUpdateCallback<UserCtx>> {
        self.should_update_callback.as_ref().map(Clone::clone)
    }

    pub fn update_callback(&self) -> Option<UniformCallback<UserCtx>> {
        self.update_callback.as_ref().map(Clone::clone)
    }

    pub fn update(&self, gl: &WebGl2RenderingContext, now: f64, user_ctx: Option<&UserCtx>) {
        let ctx = UniformContext::new(gl, now, self.uniform_location(), user_ctx);
        if let Some(update_callback) = &self.update_callback {
            (update_callback)(ctx);
        }
    }
}

impl<ProgramId: Id, UniformId: Id, UserCtx> Debug for Uniform<ProgramId, UniformId, UserCtx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Uniform")
            .field("id", &self.uniform_id)
            .field("uniform_location", &self.uniform_location)
            .finish()
    }
}
impl<ProgramId: Id, UniformId: Id, UserCtx> Hash for Uniform<ProgramId, UniformId, UserCtx> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uniform_id.hash(state);
    }
}

impl<ProgramId: Id, UniformId: Id, UserCtx> PartialEq for Uniform<ProgramId, UniformId, UserCtx> {
    fn eq(&self, other: &Self) -> bool {
        self.uniform_id == other.uniform_id && self.uniform_location == other.uniform_location
    }
}

impl<ProgramId: Id, UniformId: Id, UserCtx> Eq for Uniform<ProgramId, UniformId, UserCtx> {}
