use std::{fmt::Debug, ops::Deref, rc::Rc};

use js_sys::Function;

use crate::{CallbackWithContext, Either, UniformContext};

pub type UniformShouldUpdateCallbackInner<UserCtx> = Either<
    CallbackWithContext<dyn Fn(&UniformContext<UserCtx>) -> bool>,
    CallbackWithContext<Function>,
>;

/// Wrapper around CallbackWithContext -- allows for Default implementation to return `true` instead of false,
/// since, by default, uniforms should be updated if no custom optimization callback is provided.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UniformShouldUpdateCallback<UserCtx: Clone>(UniformShouldUpdateCallbackInner<UserCtx>);

impl<UserCtx: Clone> Deref for UniformShouldUpdateCallback<UserCtx> {
    type Target = UniformShouldUpdateCallbackInner<UserCtx>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<UserCtx: Clone> Default for UniformShouldUpdateCallback<UserCtx> {
    fn default() -> Self {
        Self(Either::new_a(CallbackWithContext::new(
            Rc::new(|_: &UniformContext<UserCtx>| true)
                as Rc<dyn Fn(&UniformContext<UserCtx>) -> bool>,
        )))
    }
}

impl<UserCtx: Clone> Debug for UniformShouldUpdateCallback<UserCtx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UniformShouldUpdateCallback")
            .field(&self.0)
            .finish()
    }
}

impl<UserCtx: Clone, F: Fn(&UniformContext<UserCtx>) -> bool + 'static> From<F>
    for UniformShouldUpdateCallback<UserCtx>
{
    fn from(callback: F) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            Rc::new(callback) as Rc<dyn Fn(&UniformContext<UserCtx>) -> bool>
        )))
    }
}

impl<UserCtx: Clone, F: Fn(&UniformContext<UserCtx>) -> bool + 'static> From<Rc<F>>
    for UniformShouldUpdateCallback<UserCtx>
{
    fn from(callback: Rc<F>) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            callback as Rc<dyn Fn(&UniformContext<UserCtx>) -> bool>,
        )))
    }
}

impl<UserCtx: Clone> From<Function> for UniformShouldUpdateCallback<UserCtx> {
    fn from(callback: Function) -> Self {
        Self(Either::new_b(CallbackWithContext::from(callback)))
    }
}
