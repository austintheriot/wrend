use crate::{BufferCreateCallbackJs, BufferCreateContext, Callback};
use std::{ops::Deref, rc::Rc};
use web_sys::WebGlBuffer;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Debug)]
pub struct BufferCreateCallback(
    Callback<dyn Fn(&BufferCreateContext) -> WebGlBuffer, BufferCreateCallbackJs>,
);

impl Deref for BufferCreateCallback {
    type Target = Callback<dyn Fn(&BufferCreateContext) -> WebGlBuffer, BufferCreateCallbackJs>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F: Fn(&BufferCreateContext) -> WebGlBuffer + 'static> From<F> for BufferCreateCallback {
    fn from(callback: F) -> Self {
        Self(Callback::new_rs(
            Rc::new(callback) as Rc<dyn Fn(&BufferCreateContext) -> WebGlBuffer>
        ))
    }
}

impl<F: Fn(&BufferCreateContext) -> WebGlBuffer + 'static> From<Rc<F>> for BufferCreateCallback {
    fn from(callback: Rc<F>) -> Self {
        Self(Callback::new_rs(
            callback as Rc<dyn Fn(&BufferCreateContext) -> WebGlBuffer>,
        ))
    }
}

impl From<BufferCreateCallbackJs> for BufferCreateCallback {
    fn from(callback: BufferCreateCallbackJs) -> Self {
        Self(Callback::new_js(callback))
    }
}
