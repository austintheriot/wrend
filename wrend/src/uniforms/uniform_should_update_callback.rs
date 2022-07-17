use std::{ops::Deref, rc::Rc};

use crate::{CallbackWithContext, UniformContext};

/// Wrapper around CallbackWithContext -- allows for Default implementation to return `true` instead of false,
/// since, by default, uniforms should be updated if no custom optimization callback is provided.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct UniformShouldUpdateCallback<UserCtx: Clone>(
    CallbackWithContext<UniformContext<UserCtx>, bool>,
);

impl<UserCtx: Clone> Deref for UniformShouldUpdateCallback<UserCtx> {
    type Target = CallbackWithContext<UniformContext<UserCtx>, bool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<UserCtx: Clone> Default for UniformShouldUpdateCallback<UserCtx> {
    fn default() -> Self {
        Self(CallbackWithContext::new(Rc::new(|_| true)))
    }
}
