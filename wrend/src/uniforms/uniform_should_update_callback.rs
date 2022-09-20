use std::{fmt::Debug, ops::Deref, rc::Rc};

use crate::{Callback, UniformContext, UniformShouldUpdateCallbackJs};

/// This is the inner type that [`UniformShouldUpdateCallback`] stores
pub type UniformShouldUpdateCallbackInner =
    Callback<dyn Fn(&UniformContext) -> bool, UniformShouldUpdateCallbackJs>;

/// This callback is used to determine whether a [`crate::Uniform`] should be updated or not.
/// Can be created by converting from either a Rust or a JavaScript callback.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd)]
pub struct UniformShouldUpdateCallback(UniformShouldUpdateCallbackInner);

impl Deref for UniformShouldUpdateCallback {
    type Target = UniformShouldUpdateCallbackInner;

    fn deref(&self) -> &Self::Target {
        &self.0
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
        Self(Callback::new_rs(
            Rc::new(callback) as Rc<dyn Fn(&UniformContext) -> bool>
        ))
    }
}

impl<F: Fn(&UniformContext) -> bool + 'static> From<Rc<F>> for UniformShouldUpdateCallback {
    fn from(callback: Rc<F>) -> Self {
        Self(Callback::new_rs(
            callback as Rc<dyn Fn(&UniformContext) -> bool>,
        ))
    }
}

impl From<UniformShouldUpdateCallbackJs> for UniformShouldUpdateCallback {
    fn from(callback: UniformShouldUpdateCallbackJs) -> Self {
        Self(Callback::new_js(callback))
    }
}
