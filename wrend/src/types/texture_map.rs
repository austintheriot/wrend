use js_sys::Map;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const TEXTURE_MAP: &'static str = r#"
type TextureMap = Map<string, Texture>;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Map, is_type_of = JsValue::is_object, typescript_type = "TextureMap")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type TextureMap;
}
