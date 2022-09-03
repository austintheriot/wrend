use js_sys::Array;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const TEXTURE_JS_ARRAY: &'static str = r#"
type TextureJsArray = Texture[];
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Array, is_type_of = JsValue::is_object, typescript_type = "TextureJsArray")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type TextureJsArray;
}
