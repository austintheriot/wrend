use js_sys::Function;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const FRAMEBUFFER_CREATE_CALLBACK_JS: &'static str = r#"
type FramebufferCreateCallbackJs = (ctx: FramebufferCreateContext) => WebGLFramebuffer;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Function, is_type_of = JsValue::is_function, typescript_type = "FramebufferCreateCallbackJs")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type FramebufferCreateCallbackJs;
}
