use crate::{CallbackWithContext, Either, UniformContext, UniformCreateUpdateCallbackJs};
use std::fmt::Debug;
use std::{ops::Deref, rc::Rc};

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UniformCreateUpdateCallback(
    Either<
        CallbackWithContext<dyn Fn(&UniformContext)>,
        CallbackWithContext<UniformCreateUpdateCallbackJs>,
    >,
);

impl Deref for UniformCreateUpdateCallback {
    type Target = Either<
        CallbackWithContext<dyn Fn(&UniformContext)>,
        CallbackWithContext<UniformCreateUpdateCallbackJs>,
    >;

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
        Self(Either::new_a(CallbackWithContext::from(
            Rc::new(callback) as Rc<dyn Fn(&UniformContext)>
        )))
    }
}

impl<F: Fn(&UniformContext) + 'static> From<Rc<F>> for UniformCreateUpdateCallback {
    fn from(callback: Rc<F>) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            callback as Rc<dyn Fn(&UniformContext)>,
        )))
    }
}

impl From<UniformCreateUpdateCallbackJs> for UniformCreateUpdateCallback {
    fn from(callback: UniformCreateUpdateCallbackJs) -> Self {
        Self(Either::new_b(CallbackWithContext::from(callback)))
    }
}
