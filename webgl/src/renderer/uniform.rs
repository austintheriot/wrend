use super::{id::Id, uniform_context::UniformContext};
use std::hash::Hash;
use std::{fmt::Debug, rc::Rc};
use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

pub type UniformUpdateCallback<UserCtx> = Rc<dyn Fn(&UniformContext<UserCtx>)>;

#[derive(Clone)]
pub struct Uniform<ProgramId: Id, UniformId: Id, UserCtx> {
    program_id: ProgramId,
    uniform_id: UniformId,
    uniform_location: WebGlUniformLocation,
    update_callback: UniformUpdateCallback<UserCtx>,
}

impl<ProgramId: Id, UniformId: Id, UserCtx> Uniform<ProgramId, UniformId, UserCtx> {
    // @todo move into builder pattern
    pub fn new(
        program_id: ProgramId,
        uniform_id: UniformId,
        uniform_location: WebGlUniformLocation,
        update_callback: UniformUpdateCallback<UserCtx>,
    ) -> Self {
        Self {
            program_id,
            uniform_id,
            uniform_location,
            update_callback,
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
    pub fn update(&self, gl: &WebGl2RenderingContext, now: f64, user_ctx: Option<&UserCtx>) {
        let ctx = UniformContext::new(gl, now, self.uniform_location(), user_ctx);
        (self.update_callback)(&ctx);
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
        self.uniform_id == other.uniform_id
            && self.uniform_location == other.uniform_location
            && Rc::ptr_eq(&self.update_callback, &other.update_callback)
    }
}

impl<ProgramId: Id, UniformId: Id, UserCtx> Eq for Uniform<ProgramId, UniformId, UserCtx> {}
