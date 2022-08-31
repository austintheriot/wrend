use js_sys::Function;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const UNIFORM_SHOULD_UPDATE_CALLBACK_JS: &'static str = r#"
type UniformShouldUpdateCallbackJs = (ctx: UniformContext) => boolean;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Function, is_type_of = JsValue::is_function, typescript_type = "UniformShouldUpdateCallbackJs")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type UniformShouldUpdateCallbackJs;
}
