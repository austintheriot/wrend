use js_sys::Array;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const STRING_ARRAY: &'static str = r#"
type StringArray = string[];
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Array, is_type_of = JsValue::is_object, typescript_type = "StringArray")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type StringArray;
}
