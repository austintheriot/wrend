use js_sys::Map;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const UNIFORM_MAP: &'static str = r#"
type UniformMap = Map<string, Uniform>;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Map, is_type_of = JsValue::is_object, typescript_type = "UniformMap")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type UniformMap;
}
