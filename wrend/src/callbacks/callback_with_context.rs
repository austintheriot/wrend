use js_sys::Function;
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::Deref,
    rc::Rc,
};
use uuid::Uuid;
use wasm_bindgen::JsValue;

/// Type alias for the default return type of the callback type
pub type CallbackWithContextDefaultReturnType = ();

/// This is a reusable callback type, useful in contexts where a callback must
/// be received from the user which will later be invoked with specific arguments.
pub struct CallbackWithContext<F: ?Sized> {
    callback: Rc<F>,
    uuid: Uuid,
}

impl CallbackWithContext<Function> {
    pub fn call(&self, arg: impl Deref<Target = JsValue>) -> Result<JsValue, JsValue> {
        let this = JsValue::NULL;
        self.deref().call1(&this, arg.deref())
    }
}

impl<F: ?Sized> CallbackWithContext<F> {
    pub fn new(callback: impl Into<CallbackWithContext<F>>) -> CallbackWithContext<F> {
        callback.into()
    }
}

impl<A, R: Default> Default for CallbackWithContext<dyn Fn(A) -> R> {
    fn default() -> Self {
        Self {
            callback: Rc::new(|_| R::default()),
            uuid: Uuid::new_v4(),
        }
    }
}

impl<F: ?Sized> Deref for CallbackWithContext<F> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &*self.callback
    }
}

impl<F: ?Sized> PartialEq for CallbackWithContext<F> {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl<F: ?Sized> Eq for CallbackWithContext<F> {}

impl<F: ?Sized> PartialOrd for CallbackWithContext<F> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.uuid.partial_cmp(&other.uuid)
    }
}

impl<F: ?Sized> Ord for CallbackWithContext<F> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.uuid.cmp(&other.uuid)
    }
}

impl<F: ?Sized> Hash for CallbackWithContext<F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl<F: ?Sized> Clone for CallbackWithContext<F> {
    fn clone(&self) -> Self {
        Self {
            callback: Rc::clone(&self.callback),
            uuid: self.uuid,
        }
    }
}

impl<F: ?Sized> Debug for CallbackWithContext<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallbackWithContext")
            .field("callback", &"[callback]")
            .field("uuid", &self.uuid)
            .finish()
    }
}

impl<F: Sized> From<F> for CallbackWithContext<F> {
    fn from(callback: F) -> Self {
        Self {
            callback: Rc::new(callback),
            uuid: Uuid::new_v4(),
        }
    }
}

impl<F: ?Sized> From<Rc<F>> for CallbackWithContext<F> {
    fn from(callback: Rc<F>) -> Self {
        Self {
            callback,
            uuid: Uuid::new_v4(),
        }
    }
}
