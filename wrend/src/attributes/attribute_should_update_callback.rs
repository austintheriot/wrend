use crate::{CallbackWithContext, AttributeContext};

pub type AttributeShouldUpdateCallback<UserCtx> =
    CallbackWithContext<AttributeContext<UserCtx>, bool>;
