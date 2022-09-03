use crate::{AttributeCreateCallbackJs, AttributeCreateContext, Callback};
use std::{ops::Deref, rc::Rc};

pub type AttributeCreateCallbackInner =
    Callback<dyn Fn(&AttributeCreateContext), AttributeCreateCallbackJs>;

#[derive(Clone, Hash, Eq, PartialOrd, Debug)]
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

impl<F: Fn(&AttributeCreateContext) + 'static> From<F> for AttributeCreateCallback {
    fn from(callback: F) -> Self {
        Self(Callback::new_rs(
            Rc::new(callback) as Rc<dyn Fn(&AttributeCreateContext)>
        ))
    }
}

impl<F: Fn(&AttributeCreateContext) + 'static> From<Rc<F>> for AttributeCreateCallback {
    fn from(callback: Rc<F>) -> Self {
        Self(Callback::new_rs(
            callback as Rc<dyn Fn(&AttributeCreateContext)>,
        ))
    }
}

impl From<AttributeCreateCallbackJs> for AttributeCreateCallback {
    fn from(callback: AttributeCreateCallbackJs) -> Self {
        Self(Callback::new_js(callback))
    }
}
