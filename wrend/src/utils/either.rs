use std::ops::Deref;

use js_sys::Function;
use log::error;
use wasm_bindgen::{JsCast, JsValue};

use crate::{CallbackWithContext, IntoJsWrapper};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Either<L, R> {
    A(L),
    B(R),
}

impl<L, R> Either<L, R> {
    pub fn new_a(a: L) -> Self {
        Either::A(a)
    }

    pub fn new_b(b: R) -> Self {
        Either::B(b)
    }

    pub fn is_a(&self) -> bool {
        match self {
            Either::A(_) => true,
            Either::B(_) => false,
        }
    }

    pub fn is_b(&self) -> bool {
        match self {
            Either::A(_) => false,
            Either::B(_) => true,
        }
    }

    pub fn a(&self) -> Option<&L> {
        match self {
            Either::A(a) => Some(a),
            Either::B(_) => None,
        }
    }

    pub fn b(&self) -> Option<&R> {
        match self {
            Either::A(_) => None,
            Either::B(b) => Some(b),
        }
    }

    pub fn unwrap_a(self) -> L {
        match self {
            Either::A(a) => a,
            Either::B(_) => panic!("called `Either::unwrap_a()` on a `B` value"),
        }
    }

    pub fn unwrap_b(self) -> R {
        match self {
            Either::A(_) => panic!("called `Either::unwrap_b()` on an `A` value"),
            Either::B(b) => b,
        }
    }
}

impl<JsFunction: Clone + Into<JsValue>, F: ?Sized>
    Either<CallbackWithContext<F>, CallbackWithContext<JsFunction>>
{
    /// Extract the JavaScript function out of a Rust / JavaScript `Either` callback, if it exists
    pub fn js_function(&self) -> Option<JsFunction> {
        self.b().map(Deref::deref).map(Clone::clone)
    }

    /// Extract the Rust function out of a Rust / JavaScript `Either` callback, if it exists
    pub fn rust_function(&self) -> Option<&F> {
        self.a().map(Deref::deref)
    }
}

impl<JsFunction: AsRef<Function>, A>
    Either<CallbackWithContext<dyn Fn(A)>, CallbackWithContext<JsFunction>>
{
    /// Makes an `Either` that is holding a Rust callback or a JavaScript callback callable as a single unit,
    /// rather than having to match on `Either` every single time to call it.
    ///
    /// It is expected by convention that the Rust callback will take the `A` variant and the JavaScript
    /// callback with take the `B` variant.
    ///
    /// @todo: If I'm feeling clever some time: find a way to make this straightforward function call
    /// for the callee, potentially by implementing Deref, by implementing Fn() (experimentally), or by
    /// returning dyn Fn closure that returns the unified return type
    pub fn call(&self, a: A) {
        match &*self {
            Either::A(rust_callback) => (rust_callback)(a),
            Either::B(js_callback) => {
                js_callback
                    .as_ref()
                    .call0(&JsValue::NULL)
                    .expect("JavaScript callback produced an error when called");
            }
        }
    }
}

impl<JsFunction: AsRef<Function>, A, R: JsCast>
    Either<CallbackWithContext<dyn Fn(A) -> R>, CallbackWithContext<JsFunction>>
{
    /// See implementation of `call` for [Either](crate::Either)
    ///
    /// This is the same function, except with the ability to return a particular value from the callback
    pub fn call_with_return(&self, a: A) -> R {
        match &*self {
            Either::A(rust_callback) => (rust_callback)(a),
            Either::B(js_callback) => {
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

impl<JsFunction: AsRef<Function>, A: Into<JsValue>>
    Either<CallbackWithContext<dyn Fn(A)>, CallbackWithContext<JsFunction>>
{
    /// See implementation of `call` for [Either](crate::Either)
    ///
    /// This is the same function, except the JavaScript callback is also called with the the same value
    /// as the Rust callback, which requires converting it into a JsValue
    pub fn call_with_arg(&self, a: A) {
        match &*self {
            Either::A(rust_callback) => (rust_callback)(a),
            Either::B(js_callback) => {
                js_callback
                    .as_ref()
                    .call1(&JsValue::NULL, &a.into())
                    .expect("JavaScript callback produced an error when called");
            }
        }
    }
}

impl<
        JsFunction: AsRef<Function>,
        JsWrapper: Into<JsValue>,
        A: IntoJsWrapper<Result = JsWrapper>,
    > Either<CallbackWithContext<dyn Fn(A)>, CallbackWithContext<JsFunction>>
{
    /// See implementation of `call` for [Either](crate::Either)
    ///
    /// This is the same function, except the JavaScript callback is also with the Rust value,
    /// after converting the Rust value into a JavaScript-compatible type.
    pub fn call_with_arg_into_js_value(&self, a: A) {
        match &*self {
            crate::Either::A(rust_callback) => (rust_callback)(a),
            crate::Either::B(js_callback) => {
                let js_wrapper: JsWrapper = a.into_js_wrapper();
                match js_callback
                    .as_ref()
                    .call1(&JsValue::NULL, &js_wrapper.into())
                {
                    Ok(_) => {},
                    Err(err) => {
                        error!("JavaScript function threw an error: {err:?}")
                    }
                }
            }
        }
    }
}

impl<
        JsFunction: AsRef<Function>,
        JsWrapper: Into<JsValue>,
        A: IntoJsWrapper<Result = JsWrapper>,
        R: JsCast,
    > Either<CallbackWithContext<dyn Fn(A) -> R>, CallbackWithContext<JsFunction>>
{
    /// See implementation of `call` for [Either](crate::Either)
    ///
    /// This is the same function, except the JavaScript callback is also with the Rust value,
    /// after converting the Rust value into a JavaScript-compatible type.
    ///
    /// Returns the resulting value from the function call.
    pub fn call_with_arg_into_js_value_and_return(&self, a: A) -> R {
        match &*self {
            crate::Either::A(rust_callback) => (rust_callback)(a),
            crate::Either::B(js_callback) => {
                let js_wrapper: JsWrapper = a.into_js_wrapper();
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

impl<JsFunction: AsRef<Function>, A: Into<JsValue>, R: JsCast>
    Either<CallbackWithContext<dyn Fn(A) -> R>, CallbackWithContext<JsFunction>>
{
    /// See implementation of `call` for [Either](crate::Either)
    ///
    /// This is the same function, except the JavaScript callback is also called with the the same value
    /// as the Rust callback, which requires converting it into a JsValue.
    ///
    /// It also returns whatever value was produced from either callback.
    pub fn call_with_arg_and_return(&self, a: A) -> R {
        match &*self {
            Either::A(rust_callback) => (rust_callback)(a),
            Either::B(js_callback) => {
                let result = js_callback
                    .as_ref()
                    .call1(&JsValue::NULL, &a.into())
                    .expect("JavaScript callback produced an error when called");
                let return_value: R = result
                    .dyn_into()
                    .expect("JsValue could not be converted to the expected type");
                return_value
            }
        }
    }
}
