use std::{fmt::Debug, ops::Deref, rc::Rc};

use crate::{CallbackWithContext, Either, UniformContext, UniformShouldUpdateCallbackJs};

pub type UniformShouldUpdateCallbackInner = Either<
    CallbackWithContext<dyn Fn(&UniformContext) -> bool>,
    CallbackWithContext<UniformShouldUpdateCallbackJs>,
>;

/// Wrapper around CallbackWithContext -- allows for Default implementation to return `true` instead of false,
/// since, by default, uniforms should be updated if no custom optimization callback is provided.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UniformShouldUpdateCallback(UniformShouldUpdateCallbackInner);

impl Deref for UniformShouldUpdateCallback {
    type Target = UniformShouldUpdateCallbackInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for UniformShouldUpdateCallback {
    fn default() -> Self {
        Self(Either::new_a(CallbackWithContext::new(
            Rc::new(|_: &UniformContext| true) as Rc<dyn Fn(&UniformContext) -> bool>,
        )))
    }
}

impl Debug for UniformShouldUpdateCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UniformShouldUpdateCallback")
            .field(&self.0)
            .finish()
    }
}

impl<F: Fn(&UniformContext) -> bool + 'static> From<F> for UniformShouldUpdateCallback {
    fn from(callback: F) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            Rc::new(callback) as Rc<dyn Fn(&UniformContext) -> bool>
        )))
    }
}

impl<F: Fn(&UniformContext) -> bool + 'static> From<Rc<F>> for UniformShouldUpdateCallback {
    fn from(callback: Rc<F>) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            callback as Rc<dyn Fn(&UniformContext) -> bool>,
        )))
    }
}

impl From<UniformShouldUpdateCallbackJs> for UniformShouldUpdateCallback {
    fn from(callback: UniformShouldUpdateCallbackJs) -> Self {
        Self(Either::new_b(CallbackWithContext::from(callback)))
    }
}
