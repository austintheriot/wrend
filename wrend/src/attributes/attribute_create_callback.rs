use crate::{AttributeCreateContext, CallbackWithContext};

pub type AttributeCreateCallback<UserCtx> = CallbackWithContext<AttributeCreateContext<UserCtx>>;
