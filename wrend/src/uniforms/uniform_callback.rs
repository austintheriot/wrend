use crate::UniformContext;
use std::{ops::Deref, rc::Rc};

pub struct UniformCallback<UserCtx>(Rc<dyn Fn(&UniformContext<UserCtx>)>);

impl<UserCtx> UniformCallback<UserCtx> {
    pub fn new(uniform_callback: Rc<dyn Fn(&UniformContext<UserCtx>)>) -> UniformCallback<UserCtx> {
        UniformCallback(uniform_callback)
    }
}

/// By default, all uniforms should be updated, because this checking if it should be updated
/// is an optional (i.e. opt-in) optimization.
impl<UserCtx> Default for UniformCallback<UserCtx> {
    fn default() -> Self {
        Self(Rc::new(|_| {}))
    }
}

impl<UserCtx> Deref for UniformCallback<UserCtx> {
    type Target = dyn Fn(&UniformContext<UserCtx>);

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<UserCtx> Clone for UniformCallback<UserCtx> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<UserCtx> From<Rc<dyn Fn(&UniformContext<UserCtx>)>> for UniformCallback<UserCtx> {
    fn from(callback: Rc<dyn Fn(&UniformContext<UserCtx>)>) -> Self {
        UniformCallback(callback)
    }
}
