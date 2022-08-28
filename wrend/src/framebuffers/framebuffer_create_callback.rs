use crate::{CallbackWithContext, Either, FramebufferCreateContext};
use js_sys::Function;
use std::{ops::Deref, rc::Rc};
use web_sys::WebGlFramebuffer;

#[derive(Clone, Hash, Eq, PartialOrd, Ord, Debug)]
pub struct FramebufferCreateCallback<UserCtx: Clone + 'static>(
    Either<
        CallbackWithContext<dyn Fn(&FramebufferCreateContext<UserCtx>) -> WebGlFramebuffer>,
        CallbackWithContext<Function>,
    >,
);

impl<UserCtx: Clone> PartialEq for FramebufferCreateCallback<UserCtx> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<UserCtx: Clone + 'static> Deref for FramebufferCreateCallback<UserCtx> {
    type Target = Either<
        CallbackWithContext<dyn Fn(&FramebufferCreateContext<UserCtx>) -> WebGlFramebuffer>,
        CallbackWithContext<Function>,
    >;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<UserCtx: Clone, F: Fn(&FramebufferCreateContext<UserCtx>) -> WebGlFramebuffer + 'static>
    From<F> for FramebufferCreateCallback<UserCtx>
{
    fn from(callback: F) -> Self {
        Self(Either::new_a(CallbackWithContext::from(Rc::new(callback)
            as Rc<
                dyn Fn(&FramebufferCreateContext<UserCtx>) -> WebGlFramebuffer,
            >)))
    }
}

impl<UserCtx: Clone, F: Fn(&FramebufferCreateContext<UserCtx>) -> WebGlFramebuffer + 'static>
    From<Rc<F>> for FramebufferCreateCallback<UserCtx>
{
    fn from(callback: Rc<F>) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            callback as Rc<dyn Fn(&FramebufferCreateContext<UserCtx>) -> WebGlFramebuffer>,
        )))
    }
}

impl<UserCtx: Clone> From<Function> for FramebufferCreateCallback<UserCtx> {
    fn from(callback: Function) -> Self {
        Self(Either::new_b(CallbackWithContext::from(callback)))
    }
}
