use crate::{BufferCreateContext, CallbackWithContext, Either};
use js_sys::Function;
use std::{ops::Deref, rc::Rc};
use web_sys::WebGlBuffer;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BufferCreateCallback(
    Either<
        CallbackWithContext<dyn Fn(&BufferCreateContext) -> WebGlBuffer>,
        CallbackWithContext<Function>,
    >,
);

impl Deref for BufferCreateCallback {
    type Target = Either<
        CallbackWithContext<dyn Fn(&BufferCreateContext) -> WebGlBuffer>,
        CallbackWithContext<Function>,
    >;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// allows this specific closure type to be received for `BufferLink`s
impl<F: Fn(&BufferCreateContext) -> WebGlBuffer + 'static> From<F>
    for BufferCreateCallback
{
    fn from(callback: F) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            Rc::new(callback) as Rc<dyn Fn(&BufferCreateContext) -> WebGlBuffer>
        )))
    }
}

impl<F: Fn(&BufferCreateContext) -> WebGlBuffer + 'static> From<Rc<F>>
    for BufferCreateCallback
{
    fn from(callback: Rc<F>) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            callback as Rc<dyn Fn(&BufferCreateContext) -> WebGlBuffer>,
        )))
    }
}

impl From<Function> for BufferCreateCallback {
    fn from(callback: Function) -> Self {
        Self(Either::new_b(CallbackWithContext::new(callback)))
    }
}
