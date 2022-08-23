use crate::Uniform;
use js_sys::Object;
use wasm_bindgen::prelude::wasm_bindgen;

pub type JsUniformInner = Uniform<String, String, Object>;

#[wasm_bindgen]
pub struct JsUniform(JsUniformInner);

impl From<JsUniformInner> for JsUniform {
    fn from(js_uniform_inner: JsUniformInner) -> Self {
        Self(js_uniform_inner)
    }
}

impl From<&JsUniformInner> for JsUniform {
    fn from(js_uniform_inner: &JsUniformInner) -> Self {
        Self(js_uniform_inner.to_owned())
    }
}
