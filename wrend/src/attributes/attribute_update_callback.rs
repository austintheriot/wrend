use crate::{AttributeContext, CallbackWithContext};

pub type AttributeUpdateCallback<UserCtx> = CallbackWithContext<AttributeContext<UserCtx>>;
