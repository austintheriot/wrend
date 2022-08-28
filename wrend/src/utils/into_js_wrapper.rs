use wasm_bindgen::JsValue;

/// For structs that were originally implemented in Rust but also
/// have a dedicated JavaScript-compatible wrapper.
pub trait IntoJsWrapper {
    type Result: Into<JsValue>;

    fn into_js_wrapper(self) -> Self::Result;
}
