use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::Deref,
    rc::Rc,
};

use uuid::Uuid;

/// Default return type of the callback type
pub type CallbackWithContextDefaultReturnType = ();

/// Alias for the inner callback type expected by `CallbackWithContext`
pub type CallbackWithContextFnType<Ctx, Returns = CallbackWithContextDefaultReturnType> =
    dyn Fn(&Ctx) -> Returns;

#[derive(Clone)]
/// Wrapper around a callback to give it a static lifetime and more easily move it around in memory
pub struct CallbackWithContext<Ctx, Returns = CallbackWithContextDefaultReturnType> {
    callback: Rc<CallbackWithContextFnType<Ctx, Returns>>,
    uuid: Uuid,
}

impl<Ctx> CallbackWithContext<Ctx> {
    pub fn new(callback: Rc<CallbackWithContextFnType<Ctx>>) -> CallbackWithContext<Ctx> {
        CallbackWithContext {
            callback,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn to_owned_callback(&self) -> Rc<CallbackWithContextFnType<Ctx>> {
        Rc::clone(&self.callback)
    }
}

impl<Ctx> Default for CallbackWithContext<Ctx> {
    fn default() -> Self {
        Self {
            callback: Rc::new(|_| {}),
            uuid: Uuid::new_v4(),
        }
    }
}

impl<Ctx> Deref for CallbackWithContext<Ctx> {
    type Target = CallbackWithContextFnType<Ctx>;

    fn deref(&self) -> &Self::Target {
        &*self.callback
    }
}

impl<Ctx> PartialEq for CallbackWithContext<Ctx> {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl<Ctx> Eq for CallbackWithContext<Ctx> {}

impl<Ctx> PartialOrd for CallbackWithContext<Ctx> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.uuid.partial_cmp(&other.uuid)
    }
}

impl<Ctx> Ord for CallbackWithContext<Ctx> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.uuid.cmp(&other.uuid)
    }
}

impl<Ctx> Hash for CallbackWithContext<Ctx> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl<Ctx> Debug for CallbackWithContext<Ctx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallbackWithContext")
            .field("callback", &"[callback function]")
            .field("uuid", &self.uuid)
            .finish()
    }
}

impl<Ctx> From<Rc<CallbackWithContextFnType<Ctx>>> for CallbackWithContext<Ctx> {
    fn from(callback: Rc<CallbackWithContextFnType<Ctx>>) -> Self {
        CallbackWithContext {
            callback,
            uuid: Uuid::new_v4(),
        }
    }
}
