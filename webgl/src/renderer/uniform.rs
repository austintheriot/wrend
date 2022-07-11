use super::{id::Id, uniform_context::UniformContext};
use std::hash::Hash;
use std::{fmt::Debug, rc::Rc};
use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

pub type UniformUpdateCallback<UserCtx> = Rc<dyn Fn(&UniformContext<UserCtx>)>;

#[derive(Clone)]
pub struct Uniform<UniformId: Id, UserCtx> {
    id: UniformId,
    uniform_location: WebGlUniformLocation,
    update_callback: UniformUpdateCallback<UserCtx>,
}

impl<UniformId: Id, UserCtx> Uniform<UniformId, UserCtx> {
    // @todo move into builder pattern
    pub fn new(
        id: UniformId,
        uniform_location: WebGlUniformLocation,
        update_callback: UniformUpdateCallback<UserCtx>,
    ) -> Self {
        Self {
            id,
            uniform_location,
            update_callback,
        }
    }

    pub fn id(&self) -> &UniformId {
        &self.id
    }
    pub fn uniform_location(&self) -> &WebGlUniformLocation {
        &self.uniform_location
    }
    pub fn update(&self, gl: &WebGl2RenderingContext, now: f64, user_ctx: Option<&UserCtx>) {
        let ctx = UniformContext::new(gl, now, self.uniform_location(), user_ctx);
        (self.update_callback)(&ctx);
    }
}

impl<UniformId: Id, UserCtx> Debug for Uniform<UniformId, UserCtx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Uniform")
            .field("id", &self.id)
            .field("uniform_location", &self.uniform_location)
            .finish()
    }
}
impl<UniformId: Id, UserCtx> Hash for Uniform<UniformId, UserCtx> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<UniformId: Id, UserCtx> PartialEq for Uniform<UniformId, UserCtx> {
    fn eq(&self, other: &Self) -> bool {
            self.id == other.id
            && self.uniform_location == other.uniform_location
            && Rc::ptr_eq(&self.update_callback, &other.update_callback)
    }
}

impl<UniformId: Id, UserCtx> Eq for Uniform<UniformId, UserCtx> {}
