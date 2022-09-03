use crate::{CallbackWithContext, IntoJsWrapper};
use js_sys::Function;
use log::error;
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    rc::Rc,
};
use wasm_bindgen::{JsCast, JsValue};

/// Wrapper type for a callback that can be either a JavaScript callback or a Rust callback
pub enum Callback<R: ?Sized, J: AsRef<Function>> {
    Rust(CallbackWithContext<R>),
    Js(CallbackWithContext<J>),
}

impl<R: ?Sized, J: AsRef<Function>> Callback<R, J> {
    pub fn new_rs(rs_callback: impl Into<CallbackWithContext<R>>) -> Self {
        Self::Rust(rs_callback.into())
    }

    pub fn new_js(callback: impl Into<CallbackWithContext<J>>) -> Self {
        Self::Js(callback.into())
    }

    pub fn js(&self) -> Option<CallbackWithContext<J>> {
        if let Callback::Js(js_callback) = &self {
            Some(js_callback.to_owned())
        } else {
            None
        }
    }

    pub fn rs(&self) -> Option<CallbackWithContext<R>> {
        if let Callback::Rust(rs_callback) = &self {
            Some(rs_callback.to_owned())
        } else {
            None
        }
    }

    pub fn js_unwrap(&self) -> CallbackWithContext<J> {
        match &self {
            Callback::Rust(rs_callback) => {
                panic!("Called `Callback::js_unwrap()` on a `Callback::Rust(_)` value {rs_callback:#?}")
            }
            Callback::Js(js_callback) => js_callback.to_owned(),
        }
    }

    pub fn rs_unwrap(&self) -> CallbackWithContext<R> {
        match &self {
            Callback::Rust(rs_callback) => rs_callback.to_owned(),
            Callback::Js(js_callback) => {
                panic!("Called `Callback::rust_unwrap()` on a `Callback::Js(_)` value {js_callback:#?}")
            }
        }
    }
}

impl<R: ?Sized, J: AsRef<Function> + Clone> Callback<R, J> {
    pub fn js_inner_owned(&self) -> Option<J> {
        self.js().map(|c| (*c).clone())
    }
}

impl<R: ?Sized + Clone, J: AsRef<Function>> Callback<R, J> {
    pub fn rs_inner_owned(&self) -> Option<R> {
        self.rs().map(|c| (*c).clone())
    }
}

impl<J: AsRef<Function>> Callback<dyn Fn(), J> {
    /// If the function is a rust callback, the argument is supplied, otherwise the javascript function
    /// is called without any arguments.
    pub fn call_with_no_arg(&self) {
        match &self {
            Callback::Rust(rs_callback) => (rs_callback)(),
            Callback::Js(js_callback) => {
                if let Err(err) = js_callback.as_ref().call0(&JsValue::NULL) {
                    error!("JavaScript callback produced an error when called: {err:?}")
                }
            }
        }
    }
}

impl<A, J: AsRef<Function>> Callback<dyn Fn(A), J> {
    /// If the function is a rust callback, the argument is supplied, otherwise the javascript function
    /// is called without any arguments.
    pub fn call_with_rust_arg(&self, a: A) {
        match &self {
            Callback::Rust(rs_callback) => (rs_callback)(a),
            Callback::Js(js_callback) => {
                if let Err(err) = js_callback.as_ref().call0(&JsValue::NULL) {
                    error!("JavaScript callback produced an error when called: {err:?}")
                }
            }
        }
    }
}

impl<A, R: JsCast, J: AsRef<Function>> Callback<dyn Fn(A) -> R, J> {
    /// If the function is a rust callback, the argument is supplied, otherwise the javascript function
    /// is called without any arguments.
    ///
    /// Returns the result value
    pub fn call_with_rust_arg_and_return(&self, a: A) -> R {
        match &self {
            Callback::Rust(rs_callback) => (rs_callback)(a),
            Callback::Js(js_callback) => {
                let result = js_callback
                    .as_ref()
                    .call0(&JsValue::NULL)
                    .expect("JavaScript callback produced an error when called");
                let return_value: R = result
                    .dyn_into()
                    .expect("JsValue could not be converted to the expected type");
                return_value
            }
        }
    }
}

impl<A: Into<JsValue>, J: AsRef<Function>> Callback<dyn Fn(A), J> {
    /// Calls either underlying callback with the argument supplied
    pub fn call_with_js_arg(&self, a: A) {
        match &self {
            Callback::Rust(rs_callback) => (rs_callback)(a),
            Callback::Js(js_callback) => {
                if let Err(err) = js_callback.as_ref().call1(&JsValue::NULL, &a.into()) {
                    error!("JavaScript callback produced an error when called: {err:?}")
                }
            }
        }
    }
}

impl<W: Into<JsValue>, A: IntoJsWrapper<Result = W>, J: AsRef<Function>> Callback<dyn Fn(A), J> {
    /// Calls either underlying callback with the argument supplied
    pub fn call_with_into_js_arg(&self, a: A) {
        match &self {
            Callback::Rust(rs_callback) => (rs_callback)(a),
            Callback::Js(js_callback) => {
                let js_wrapper: W = a.into_js_wrapper();
                if let Err(err) = js_callback
                    .as_ref()
                    .call1(&JsValue::NULL, &js_wrapper.into())
                {
                    error!("JavaScript callback produced an error when called: {err:?}")
                }
            }
        }
    }
}

impl<A: AsRef<JsValue>, J: AsRef<Function>> Callback<dyn Fn(A), J> {
    /// Calls either underlying callback with the argument supplied
    pub fn call_with_as_js_arg(&self, a: A) {
        match &self {
            Callback::Rust(rs_callback) => (rs_callback)(a),
            Callback::Js(js_callback) => {
                if let Err(err) = js_callback.as_ref().call1(&JsValue::NULL, a.as_ref()) {
                    error!("JavaScript callback produced an error when called: {err:?}")
                }
            }
        }
    }
}

impl<W: Into<JsValue>, A: IntoJsWrapper<Result = W>, R: JsCast, J: AsRef<Function>>
    Callback<dyn Fn(A) -> R, J>
{
    /// Calls either underlying callback with the argument supplied
    /// and returns the resulting value
    pub fn call_with_into_js_arg_and_return(&self, a: A) -> R {
        match &self {
            Callback::Rust(rs_callback) => (rs_callback)(a),
            Callback::Js(js_callback) => {
                let js_wrapper: W = a.into_js_wrapper();
                let result = js_callback
                    .as_ref()
                    .call1(&JsValue::NULL, &js_wrapper.into())
                    .expect("JavaScript callback produced an error when called");
                let return_value: R = result
                    .dyn_into()
                    .expect("JsValue could not be converted to the expected type");
                return_value
            }
        }
    }
}

impl<R: Sized, J: AsRef<Function>> From<R> for Callback<R, J> {
    fn from(callback: R) -> Self {
        Self::Rust(CallbackWithContext::new(callback))
    }
}

impl<R: ?Sized, J: AsRef<Function>> From<Rc<R>> for Callback<R, J> {
    fn from(callback: Rc<R>) -> Self {
        Self::Rust(CallbackWithContext::from(callback))
    }
}

impl<R: ?Sized, J: AsRef<Function>> PartialEq for Callback<R, J> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Callback::Rust(l0), Callback::Rust(r0)) => l0 == r0,
            (Callback::Rust(_), Callback::Js(_)) => false,
            (Callback::Js(_), Callback::Rust(_)) => false,
            (Callback::Js(l0), Callback::Js(r0)) => l0 == r0,
        }
    }
}

impl<R: ?Sized, J: AsRef<Function>> Eq for Callback<R, J> {}

impl<R: ?Sized, J: AsRef<Function>> PartialOrd for Callback<R, J> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Callback::Rust(l0), Callback::Rust(r0)) => Some(l0.cmp(r0)),
            (Callback::Rust(_), Callback::Js(_)) => None,
            (Callback::Js(_), Callback::Rust(_)) => None,
            (Callback::Js(l0), Callback::Js(r0)) => Some(l0.cmp(r0)),
        }
    }
}

impl<R: ?Sized, J: AsRef<Function>> Clone for Callback<R, J> {
    fn clone(&self) -> Self {
        match self {
            Self::Rust(rs_callback) => Self::Rust(rs_callback.clone()),
            Self::Js(js_callback) => Self::Js(js_callback.clone()),
        }
    }
}

impl<R: ?Sized, J: AsRef<Function>> Hash for Callback<R, J> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Self::Rust(rs_callback) => rs_callback.hash(state),
            Self::Js(js_callback) => js_callback.hash(state),
        }
    }
}

impl<R: ?Sized, J: AsRef<Function>> Debug for Callback<R, J> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rust(rs_callback) => f.debug_tuple("Rust").field(rs_callback).finish(),
            Self::Js(js_callback) => f.debug_tuple("Js").field(js_callback).finish(),
        }
    }
}
