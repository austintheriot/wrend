use js_sys::Function;

use crate::{CallbackWithContext, Either, UniformContext};
use std::fmt::Debug;
use std::{ops::Deref, rc::Rc};

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UniformCreateUpdateCallback<UserCtx>(
    Either<CallbackWithContext<dyn Fn(&UniformContext<UserCtx>)>, CallbackWithContext<Function>>,
);

impl<UserCtx> Deref for UniformCreateUpdateCallback<UserCtx> {
    type Target = Either<
        CallbackWithContext<dyn Fn(&UniformContext<UserCtx>)>,
        CallbackWithContext<Function>,
    >;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<UserCtx> Debug for UniformCreateUpdateCallback<UserCtx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UniformCreateUpdateCallback")
            .field(&self.0)
            .finish()
    }
}

impl<UserCtx, F: Fn(&UniformContext<UserCtx>) + 'static> From<F>
    for UniformCreateUpdateCallback<UserCtx>
{
    fn from(callback: F) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            Rc::new(callback) as Rc<dyn Fn(&UniformContext<UserCtx>)>
        )))
    }
}

impl<UserCtx, F: Fn(&UniformContext<UserCtx>) + 'static> From<Rc<F>>
    for UniformCreateUpdateCallback<UserCtx>
{
    fn from(callback: Rc<F>) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            callback as Rc<dyn Fn(&UniformContext<UserCtx>)>,
        )))
    }
}

impl<UserCtx> From<Function> for UniformCreateUpdateCallback<UserCtx> {
    fn from(callback: Function) -> Self {
        Self(Either::new_b(CallbackWithContext::from(callback)))
    }
}
