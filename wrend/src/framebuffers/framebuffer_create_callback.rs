use crate::{CallbackWithContext, FramebufferCreateContext};
use web_sys::WebGlFramebuffer;

pub type FramebufferCreateCallback<UserCtx> =
    CallbackWithContext<FramebufferCreateContext<UserCtx>, WebGlFramebuffer>;
