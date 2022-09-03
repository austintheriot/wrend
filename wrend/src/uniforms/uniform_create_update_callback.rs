use crate::{Callback, UniformContext, UniformCreateUpdateCallbackJs};
use std::fmt::Debug;
use std::{ops::Deref, rc::Rc};

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd)]
pub struct UniformCreateUpdateCallback(
    Callback<dyn Fn(&UniformContext), UniformCreateUpdateCallbackJs>,
);

impl Deref for UniformCreateUpdateCallback {
    type Target = Callback<dyn Fn(&UniformContext), UniformCreateUpdateCallbackJs>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for UniformCreateUpdateCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UniformCreateUpdateCallback")
            .field(&self.0)
            .finish()
    }
}

impl<F: Fn(&UniformContext) + 'static> From<F> for UniformCreateUpdateCallback {
    fn from(callback: F) -> Self {
        Self(Callback::new_rs(
            Rc::new(callback) as Rc<dyn Fn(&UniformContext)>
        ))
    }
}

impl<F: Fn(&UniformContext) + 'static> From<Rc<F>> for UniformCreateUpdateCallback {
    fn from(callback: Rc<F>) -> Self {
        Self(Callback::new_rs(callback as Rc<dyn Fn(&UniformContext)>))
    }
}

impl From<UniformCreateUpdateCallbackJs> for UniformCreateUpdateCallback {
    fn from(callback: UniformCreateUpdateCallbackJs) -> Self {
        Self(Callback::new_js(callback))
    }
}
