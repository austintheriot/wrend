use js_sys::Map;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const WEBGL_SHADER_MAP: &'static str = r#"
type WebGlShaderMap = Map<string, WebGLShader>;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Map, is_type_of = JsValue::is_object, typescript_type = "WebGlShaderMap")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type WebGlShaderMap;
}
