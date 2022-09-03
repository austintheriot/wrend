use js_sys::Array;
use wasm_bindgen::prelude::wasm_bindgen;

// @todo: allow renderer_data to be provided as argument
#[wasm_bindgen(typescript_custom_section)]
const VAO_ID_ARRAY: &'static str = r#"
type VaoIdArray = string[];
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Array, is_type_of = JsValue::is_object, typescript_type = "VaoIdArray")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type VaoIdArray;
}
