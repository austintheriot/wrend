use crate::{BufferCreateContext, CallbackWithContext};
use std::{ops::Deref, rc::Rc};
use web_sys::WebGlBuffer;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BufferCreateCallback<UserCtx>(
    CallbackWithContext<dyn Fn(&BufferCreateContext<UserCtx>) -> WebGlBuffer>,
);

impl<UserCtx> Deref for BufferCreateCallback<UserCtx> {
    type Target = CallbackWithContext<dyn Fn(&BufferCreateContext<UserCtx>) -> WebGlBuffer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// allows this specific closure type to be received for `BufferLink`s
impl<UserCtx, F: Fn(&BufferCreateContext<UserCtx>) -> WebGlBuffer + 'static> From<F>
    for BufferCreateCallback<UserCtx>
{
    fn from(callback: F) -> Self {
        Self(CallbackWithContext::from(
            Rc::new(callback) as Rc<dyn Fn(&BufferCreateContext<UserCtx>) -> WebGlBuffer>
        ))
    }
}

impl<UserCtx, F: Fn(&BufferCreateContext<UserCtx>) -> WebGlBuffer + 'static> From<Rc<F>>
    for BufferCreateCallback<UserCtx>
{
    fn from(callback: Rc<F>) -> Self {
        Self(CallbackWithContext::from(
            callback as Rc<dyn Fn(&BufferCreateContext<UserCtx>) -> WebGlBuffer>,
        ))
    }
}
