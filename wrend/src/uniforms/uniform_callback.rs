use crate::{CallbackWithContext, UniformContext};

pub type UniformCallback<UserCtx> = CallbackWithContext<UniformContext<UserCtx>>;
