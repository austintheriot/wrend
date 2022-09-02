use js_sys::Function;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const ANIMATION_CALLBACK_JS: &'static str = r#"
type AnimationCallbackJs = (renderer_data: RendererData) => void;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Function, is_type_of = JsValue::is_function, typescript_type = "AnimationCallbackJs")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type AnimationCallbackJs;
}
