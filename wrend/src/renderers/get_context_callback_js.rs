use js_sys::Function;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const GET_CONTEXT_CALLBACK_JS: &'static str = r#"
type GetContextCallbackJs = (canvas: HTMLCanvasElement) => WebGL2RenderingContext;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Function, is_type_of = JsValue::is_function, typescript_type = "GetContextCallbackJs")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type GetContextCallbackJs;
}
