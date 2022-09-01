use crate::{CallbackWithContext, Either, FramebufferCreateCallbackJs, FramebufferCreateContext};

use std::{ops::Deref, rc::Rc};
use web_sys::WebGlFramebuffer;

#[derive(Clone, Hash, Eq, PartialOrd, Ord, Debug)]
pub struct FramebufferCreateCallback(
    Either<
        CallbackWithContext<dyn Fn(&FramebufferCreateContext) -> WebGlFramebuffer>,
        CallbackWithContext<FramebufferCreateCallbackJs>,
    >,
);

impl PartialEq for FramebufferCreateCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Deref for FramebufferCreateCallback {
    type Target = Either<
        CallbackWithContext<dyn Fn(&FramebufferCreateContext) -> WebGlFramebuffer>,
        CallbackWithContext<FramebufferCreateCallbackJs>,
    >;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F: Fn(&FramebufferCreateContext) -> WebGlFramebuffer + 'static> From<F>
    for FramebufferCreateCallback
{
    fn from(callback: F) -> Self {
        Self(Either::new_a(CallbackWithContext::from(Rc::new(callback)
            as Rc<
                dyn Fn(&FramebufferCreateContext) -> WebGlFramebuffer,
            >)))
    }
}

impl<F: Fn(&FramebufferCreateContext) -> WebGlFramebuffer + 'static> From<Rc<F>>
    for FramebufferCreateCallback
{
    fn from(callback: Rc<F>) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            callback as Rc<dyn Fn(&FramebufferCreateContext) -> WebGlFramebuffer>,
        )))
    }
}

impl From<FramebufferCreateCallbackJs> for FramebufferCreateCallback {
    fn from(callback: FramebufferCreateCallbackJs) -> Self {
        Self(Either::new_b(CallbackWithContext::from(callback)))
    }
}
