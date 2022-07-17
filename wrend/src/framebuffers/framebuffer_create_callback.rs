use web_sys::WebGlFramebuffer;

use crate::{CallbackWithContext, FramebufferCreateContext};

pub type FramebufferCreateCallback<UserCtx> =
    CallbackWithContext<FramebufferCreateContext<UserCtx>, WebGlFramebuffer>;
