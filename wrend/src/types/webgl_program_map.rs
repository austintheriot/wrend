use js_sys::Map;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const WEBGL_PROGRAM_MAP: &'static str = r#"
type WebGlProgramMap = Map<string, WebGLShader>;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Map, is_type_of = JsValue::is_object, typescript_type = "WebGlProgramMap")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type WebGlProgramMap;
}
