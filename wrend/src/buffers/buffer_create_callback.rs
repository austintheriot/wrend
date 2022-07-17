use crate::{BufferCreateContext, CallbackWithContext};
use web_sys::WebGlBuffer;

pub type BufferCreateCallback<UserCtx> =
    CallbackWithContext<BufferCreateContext<UserCtx>, WebGlBuffer>;
