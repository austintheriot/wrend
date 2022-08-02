use crate::{AttributeCreateContext, CallbackWithContext};
use std::{ops::Deref, rc::Rc};

#[derive(Clone, Hash, Eq, PartialOrd, Ord, Debug)]
pub struct AttributeCreateCallback<UserCtx: Clone>(
    CallbackWithContext<dyn Fn(&AttributeCreateContext<UserCtx>)>,
);

impl<UserCtx: Clone> PartialEq for AttributeCreateCallback<UserCtx> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<UserCtx: Clone> Deref for AttributeCreateCallback<UserCtx> {
    type Target = CallbackWithContext<dyn Fn(&AttributeCreateContext<UserCtx>)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<UserCtx: Clone, F: Fn(&AttributeCreateContext<UserCtx>) + 'static> From<F>
    for AttributeCreateCallback<UserCtx>
{
    fn from(callback: F) -> Self {
        Self(CallbackWithContext::from(
            Rc::new(callback) as Rc<dyn Fn(&AttributeCreateContext<UserCtx>)>
        ))
    }
}

impl<UserCtx: Clone, F: Fn(&AttributeCreateContext<UserCtx>) + 'static> From<Rc<F>>
    for AttributeCreateCallback<UserCtx>
{
    fn from(callback: Rc<F>) -> Self {
        Self(CallbackWithContext::from(
            callback as Rc<dyn Fn(&AttributeCreateContext<UserCtx>)>,
        ))
    }
}
