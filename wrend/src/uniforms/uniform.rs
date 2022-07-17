use crate::Id;
use crate::UniformCallback;
use crate::UniformContext;
use crate::UniformShouldUpdateCallback;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlUniformLocation};

#[derive(Clone)]
pub struct Uniform<ProgramId: Id, UniformId: Id, UserCtx> {
    program_ids: Vec<ProgramId>,
    uniform_id: UniformId,
    uniform_locations: HashMap<ProgramId, WebGlUniformLocation>,
    initialize_callback: UniformCallback<UserCtx>,
    update_callback: Option<UniformCallback<UserCtx>>,
    should_update_callback: Option<UniformShouldUpdateCallback<UserCtx>>,
}

impl<ProgramId: Id, UniformId: Id, UserCtx> Uniform<ProgramId, UniformId, UserCtx> {
    // @todo move into builder pattern
    pub fn new(
        program_ids: Vec<ProgramId>,
        uniform_id: UniformId,
        // a single conceptual uniform can be shared across multiple programs and updated in tandem
        uniform_locations: HashMap<ProgramId, WebGlUniformLocation>,
        initialize_callback: UniformCallback<UserCtx>,
        update_callback: Option<UniformCallback<UserCtx>>,
        should_update_callback: Option<UniformShouldUpdateCallback<UserCtx>>,
    ) -> Self {
        Self {
            program_ids,
            uniform_id,
            uniform_locations,
            initialize_callback,
            update_callback,
            should_update_callback,
        }
    }

    pub fn program_ids(&self) -> &Vec<ProgramId> {
        &self.program_ids
    }

    pub fn uniform_id(&self) -> &UniformId {
        &self.uniform_id
    }

    pub fn uniform_locations(&self) -> &HashMap<ProgramId, WebGlUniformLocation> {
        &self.uniform_locations
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

    /// updates the uniform for every Program where this uniform is used,
    /// using the update callback that was passed in at creation time
    pub fn update(
        &self,
        gl: &WebGl2RenderingContext,
        now: f64,
        user_ctx: Option<&UserCtx>,
        programs: &HashMap<ProgramId, WebGlProgram>,
    ) {
        let uniform_locations = self.uniform_locations();

        for (program_id, uniform_location) in uniform_locations.iter() {
            let program = programs
                .get(program_id)
                .expect("Program id should correspond to a saved WebGlProgram");

            gl.use_program(Some(program));

            let ctx = UniformContext::new(gl, now, uniform_location, user_ctx);
            let should_update_callback = self.should_update_callback().unwrap_or_default();
            if let Some(update_callback) = &self.update_callback {
                if should_update_callback(&ctx) {
                    (update_callback)(&ctx)
                }
            }

            gl.use_program(None);
        }
    }
}

impl<ProgramId: Id, UniformId: Id, UserCtx> Debug for Uniform<ProgramId, UniformId, UserCtx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Uniform")
            .field("id", &self.uniform_id)
            .field("uniform_locations", &self.uniform_locations)
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
        self.uniform_id == other.uniform_id && self.uniform_locations == other.uniform_locations
    }
}

impl<ProgramId: Id, UniformId: Id, UserCtx> Eq for Uniform<ProgramId, UniformId, UserCtx> {}
