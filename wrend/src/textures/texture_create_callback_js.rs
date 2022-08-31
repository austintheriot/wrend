use js_sys::Function;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const TEXTURE_CREATE_CALLBACK_JS: &'static str = r#"
type TextureCreateCallbackJs = (canvas: TextureCreateContext) => WebGLTexture;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Function, is_type_of = JsValue::is_function, typescript_type = "TextureCreateCallbackJs")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type TextureCreateCallbackJs;
}
