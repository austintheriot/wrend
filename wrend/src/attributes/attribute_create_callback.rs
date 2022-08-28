use js_sys::Function;

use crate::{AttributeCreateContext, CallbackWithContext, Either};
use std::{ops::Deref, rc::Rc};

pub type AttributeCreateCallbackInner = Either<
    CallbackWithContext<dyn Fn(AttributeCreateContext)>,
    CallbackWithContext<Function>,
>;

#[derive(Clone, Hash, Eq, PartialOrd, Ord, Debug)]
pub struct AttributeCreateCallback(AttributeCreateCallbackInner);

impl PartialEq for AttributeCreateCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Deref for AttributeCreateCallback {
    type Target = AttributeCreateCallbackInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F: Fn(AttributeCreateContext) + 'static> From<F>
    for AttributeCreateCallback
{
    fn from(callback: F) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            Rc::new(callback) as Rc<dyn Fn(AttributeCreateContext)>
        )))
    }
}

impl<F: Fn(AttributeCreateContext) + 'static> From<Rc<F>>
    for AttributeCreateCallback
{
    fn from(callback: Rc<F>) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            callback as Rc<dyn Fn(AttributeCreateContext)>,
        )))
    }
}

impl From<Function> for AttributeCreateCallback {
    fn from(callback: Function) -> Self {
        Self(Either::new_b(CallbackWithContext::from(callback)))
    }
}
