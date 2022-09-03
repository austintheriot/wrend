use js_sys::Map;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const ATTRIBUTE_MAP: &'static str = r#"
type AttributeMap = Map<string, Attribute>;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Map, is_type_of = JsValue::is_object, typescript_type = "AttributeMap")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type AttributeMap;
}
