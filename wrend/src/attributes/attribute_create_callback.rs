use crate::{CallbackWithContext, AttributeCreateContext};

pub type AttributeCreateCallback<UserCtx> = CallbackWithContext<AttributeCreateContext<UserCtx>>;
