use crate::{Callback, FramebufferCreateCallbackJs, FramebufferCreateContext};

use std::{ops::Deref, rc::Rc};
use web_sys::WebGlFramebuffer;

#[derive(Clone, Hash, Eq, PartialOrd, Debug)]
pub struct FramebufferCreateCallback(
    Callback<dyn Fn(&FramebufferCreateContext) -> WebGlFramebuffer, FramebufferCreateCallbackJs>,
);

impl PartialEq for FramebufferCreateCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Deref for FramebufferCreateCallback {
    type Target = Callback<
        dyn Fn(&FramebufferCreateContext) -> WebGlFramebuffer,
        FramebufferCreateCallbackJs,
    >;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F: Fn(&FramebufferCreateContext) -> WebGlFramebuffer + 'static> From<F>
    for FramebufferCreateCallback
{
    fn from(callback: F) -> Self {
        Self(Callback::new_rs(
            Rc::new(callback) as Rc<dyn Fn(&FramebufferCreateContext) -> WebGlFramebuffer>
        ))
    }
}

impl<F: Fn(&FramebufferCreateContext) -> WebGlFramebuffer + 'static> From<Rc<F>>
    for FramebufferCreateCallback
{
    fn from(callback: Rc<F>) -> Self {
        Self(Callback::new_rs(
            callback as Rc<dyn Fn(&FramebufferCreateContext) -> WebGlFramebuffer>,
        ))
    }
}

impl From<FramebufferCreateCallbackJs> for FramebufferCreateCallback {
    fn from(callback: FramebufferCreateCallbackJs) -> Self {
        Self(Callback::new_js(callback))
    }
}
