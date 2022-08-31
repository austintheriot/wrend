use js_sys::Function;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const ATTRIBUTE_CREATE_CALLBACK_JS: &'static str = r#"
type AttributeCreateCallbackJs = (attribute_create_context: AttributeCreateContext) => void;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Function, is_type_of = JsValue::is_function, typescript_type = "AttributeCreateCallbackJs")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type AttributeCreateCallbackJs;
}
