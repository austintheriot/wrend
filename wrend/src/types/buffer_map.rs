use js_sys::Map;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const BUFFER_MAP: &'static str = r#"
type BufferMap = Map<string, Buffer>;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Map, is_type_of = JsValue::is_object, typescript_type = "BufferMap")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type BufferMap;
}
