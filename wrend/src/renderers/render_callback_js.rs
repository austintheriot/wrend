use js_sys::Function;
use wasm_bindgen::prelude::wasm_bindgen;

// @todo: allow renderer_data to be provided as argument
#[wasm_bindgen(typescript_custom_section)]
const RENDER_CALLBACK_JS: &'static str = r#"
type RenderCallbackJs = (renderer_data: RendererData) => void;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Function, is_type_of = JsValue::is_function, typescript_type = "RenderCallbackJs")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type RenderCallbackJs;
}
