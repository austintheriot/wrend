use wasm_bindgen::prelude::wasm_bindgen;

/// Enables printing readable error messages to the console
/// when Rust code panics.
#[wasm_bindgen(js_name = "enableErrorMessages")]
pub fn enable_error_messages() {
    console_error_panic_hook::set_once();
}
