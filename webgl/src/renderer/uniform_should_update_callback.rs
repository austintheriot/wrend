use super::uniform_context::UniformContext;
use std::{rc::Rc, ops::Deref};

pub struct UniformShouldUpdateCallback<UserCtx>(Rc<dyn Fn(UniformContext<UserCtx>) -> bool>);

/// By default, all uniforms should be updated, because this checking if it should be updated
/// is an optional optimization.
impl<UserCtx> Default for UniformShouldUpdateCallback<UserCtx> {
    fn default() -> Self {
        Self(Rc::new(|_| true))
    }
}

impl<UserCtx> Deref for UniformShouldUpdateCallback<UserCtx> {
    type Target = (dyn Fn(UniformContext<UserCtx>) -> bool);

    fn deref(&self) -> &Self::Target {
       &*self.0
    }
}


impl<UserCtx> Clone for UniformShouldUpdateCallback<UserCtx> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<UserCtx> From<Rc<dyn Fn(UniformContext<UserCtx>) -> bool>>
    for UniformShouldUpdateCallback<UserCtx>
{
    fn from(callback: Rc<dyn Fn(UniformContext<UserCtx>) -> bool>) -> Self {
        UniformShouldUpdateCallback(callback)
    }
}
