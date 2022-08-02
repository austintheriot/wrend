use crate::{CallbackWithContext, UniformContext};
use std::fmt::Debug;
use std::{ops::Deref, rc::Rc};

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UniformCallback<UserCtx>(CallbackWithContext<dyn Fn(&UniformContext<UserCtx>)>);

impl<UserCtx> Deref for UniformCallback<UserCtx> {
    type Target = CallbackWithContext<dyn Fn(&UniformContext<UserCtx>)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<UserCtx> Debug for UniformCallback<UserCtx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UniformCallback").field(&self.0).finish()
    }
}

impl<UserCtx, F: Fn(&UniformContext<UserCtx>) + 'static> From<F> for UniformCallback<UserCtx> {
    fn from(callback: F) -> Self {
        Self(CallbackWithContext::from(
            Rc::new(callback) as Rc<dyn Fn(&UniformContext<UserCtx>)>
        ))
    }
}

impl<UserCtx, F: Fn(&UniformContext<UserCtx>) + 'static> From<Rc<F>> for UniformCallback<UserCtx> {
    fn from(callback: Rc<F>) -> Self {
        Self(CallbackWithContext::from(
            callback as Rc<dyn Fn(&UniformContext<UserCtx>)>,
        ))
    }
}
