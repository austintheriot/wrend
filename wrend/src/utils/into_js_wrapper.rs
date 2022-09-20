use wasm_bindgen::JsValue;

/// For structs that were originally implemented in Rust but also
/// have a dedicated JavaScript-compatible wrapper.
/// 
/// This allows accepting Rust values as functions for a [`Callback`](crate::Callback)
pub trait IntoJsWrapper {
    /// This is the type that should returned from [`into_js_wrapper`](#method.into_js_wrapper)
    type Result: Into<JsValue>;

    /// Converts a value into another value that can be converted into a [`JsValue`]
    fn into_js_wrapper(self) -> Self::Result;
}
