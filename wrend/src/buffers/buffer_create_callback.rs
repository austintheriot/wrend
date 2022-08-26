use crate::{BufferCreateContext, CallbackWithContext, Either};
use js_sys::{Function, Object};
use std::{ops::Deref, rc::Rc};
use web_sys::WebGlBuffer;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BufferCreateCallback<UserCtx>(
    Either<
        CallbackWithContext<dyn Fn(&BufferCreateContext<UserCtx>) -> WebGlBuffer>,
        CallbackWithContext<Function>,
    >,
);

impl<UserCtx> Deref for BufferCreateCallback<UserCtx> {
    type Target = Either<
        CallbackWithContext<dyn Fn(&BufferCreateContext<UserCtx>) -> WebGlBuffer>,
        CallbackWithContext<Function>,
    >;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// allows this specific closure type to be received for `BufferLink`s
impl<UserCtx, F: Fn(&BufferCreateContext<UserCtx>) -> WebGlBuffer + 'static> From<F>
    for BufferCreateCallback<UserCtx>
{
    fn from(callback: F) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            Rc::new(callback) as Rc<dyn Fn(&BufferCreateContext<UserCtx>) -> WebGlBuffer>
        )))
    }
}

impl<UserCtx, F: Fn(&BufferCreateContext<UserCtx>) -> WebGlBuffer + 'static> From<Rc<F>>
    for BufferCreateCallback<UserCtx>
{
    fn from(callback: Rc<F>) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            callback as Rc<dyn Fn(&BufferCreateContext<UserCtx>) -> WebGlBuffer>,
        )))
    }
}

impl From<Function> for BufferCreateCallback<Object> {
    fn from(callback: Function) -> Self {
        Self(Either::new_b(CallbackWithContext::new(callback)))
    }
}
