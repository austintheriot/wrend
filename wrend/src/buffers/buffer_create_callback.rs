use crate::{BufferCreateCallbackJs, BufferCreateContext, CallbackWithContext, Either};
use std::{ops::Deref, rc::Rc};
use web_sys::WebGlBuffer;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BufferCreateCallback(
    Either<
        CallbackWithContext<dyn Fn(&BufferCreateContext) -> WebGlBuffer>,
        CallbackWithContext<BufferCreateCallbackJs>,
    >,
);

impl Deref for BufferCreateCallback {
    type Target = Either<
        CallbackWithContext<dyn Fn(&BufferCreateContext) -> WebGlBuffer>,
        CallbackWithContext<BufferCreateCallbackJs>,
    >;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F: Fn(&BufferCreateContext) -> WebGlBuffer + 'static> From<F> for BufferCreateCallback {
    fn from(callback: F) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            Rc::new(callback) as Rc<dyn Fn(&BufferCreateContext) -> WebGlBuffer>
        )))
    }
}

impl<F: Fn(&BufferCreateContext) -> WebGlBuffer + 'static> From<Rc<F>> for BufferCreateCallback {
    fn from(callback: Rc<F>) -> Self {
        Self(Either::new_a(CallbackWithContext::from(
            callback as Rc<dyn Fn(&BufferCreateContext) -> WebGlBuffer>,
        )))
    }
}

impl From<BufferCreateCallbackJs> for BufferCreateCallback {
    fn from(callback: BufferCreateCallbackJs) -> Self {
        Self(Either::new_b(CallbackWithContext::new(callback)))
    }
}
