use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::Deref,
    rc::Rc,
};
use uuid::Uuid;

/// Type alias for the default return type of the callback type
pub type CallbackWithContextDefaultReturnType = ();

/// Type alias for the inner callback type expected by `CallbackWithContext`
pub type CallbackWithContextFnType<Ctx, Returns = CallbackWithContextDefaultReturnType> =
    dyn Fn(&Ctx) -> Returns;

/// This is a reusable callback type, useful in contexts where a callback must
/// be received from the user which will later be invoked with specific arguments.
pub struct CallbackWithContext<Ctx, Returns = CallbackWithContextDefaultReturnType> {
    callback: Rc<CallbackWithContextFnType<Ctx, Returns>>,
    uuid: Uuid,
}

impl<Ctx, Return> CallbackWithContext<Ctx, Return> {
    pub fn new(
        callback: Rc<CallbackWithContextFnType<Ctx, Return>>,
    ) -> CallbackWithContext<Ctx, Return> {
        CallbackWithContext {
            callback,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn to_owned_callback(&self) -> Rc<CallbackWithContextFnType<Ctx, Return>> {
        Rc::clone(&self.callback)
    }
}

impl<Ctx, Return: Default> Default for CallbackWithContext<Ctx, Return> {
    fn default() -> Self {
        Self {
            callback: Rc::new(|_| Return::default()),
            uuid: Uuid::new_v4(),
        }
    }
}

impl<Ctx, Return> Deref for CallbackWithContext<Ctx, Return> {
    type Target = CallbackWithContextFnType<Ctx, Return>;

    fn deref(&self) -> &Self::Target {
        &*self.callback
    }
}

impl<Ctx, Return> PartialEq for CallbackWithContext<Ctx, Return> {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl<Ctx, Return> Eq for CallbackWithContext<Ctx, Return> {}

impl<Ctx, Return> PartialOrd for CallbackWithContext<Ctx, Return> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.uuid.partial_cmp(&other.uuid)
    }
}

impl<Ctx, Return> Ord for CallbackWithContext<Ctx, Return> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.uuid.cmp(&other.uuid)
    }
}

impl<Ctx, Return> Hash for CallbackWithContext<Ctx, Return> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl<Ctx, Return> Clone for CallbackWithContext<Ctx, Return> {
    fn clone(&self) -> Self {
        Self {
            callback: self.callback.clone(),
            uuid: self.uuid,
        }
    }
}

impl<Ctx, Return> Debug for CallbackWithContext<Ctx, Return> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallbackWithContext")
            .field("callback", &"[callback function]")
            .field("uuid", &self.uuid)
            .finish()
    }
}

impl<Ctx, Return> From<Rc<CallbackWithContextFnType<Ctx, Return>>>
    for CallbackWithContext<Ctx, Return>
{
    fn from(callback: Rc<CallbackWithContextFnType<Ctx, Return>>) -> Self {
        CallbackWithContext {
            callback,
            uuid: Uuid::new_v4(),
        }
    }
}

impl<Ctx: 'static, Return: 'static> From<fn(&Ctx) -> Return> for CallbackWithContext<Ctx, Return> {
    fn from(callback: fn(&Ctx) -> Return) -> Self {
        CallbackWithContext {
            callback: Rc::new(callback),
            uuid: Uuid::new_v4(),
        }
    }
}
