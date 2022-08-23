use std::ops::{Deref, DerefMut};

use js_sys::Function;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub struct JsCallbackWithContext(Function);

#[wasm_bindgen]
impl JsCallbackWithContext {
    pub fn new(callback: Function) -> Self {
        callback.into()
    }

    pub fn call(&self, value: JsValue) -> Result<JsValue, JsValue> {
        let this = JsValue::null();
        self.0.call1(&this, &value)
    }
}

impl From<Function> for JsCallbackWithContext {
    fn from(function: Function) -> Self {
        Self(function)
    }
}

impl Deref for JsCallbackWithContext {
    type Target = Function;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JsCallbackWithContext {
    fn deref_mut(&mut self) -> &mut Function {
        &mut self.0
    }
}
