use crate::Texture;
use wasm_bindgen::prelude::wasm_bindgen;

pub type JsTextureInner = Texture<String>;

#[wasm_bindgen]
pub struct JsTexture(JsTextureInner);

impl From<JsTextureInner> for JsTexture {
    fn from(js_texture_inner: JsTextureInner) -> Self {
        Self(js_texture_inner)
    }
}

impl From<&JsTextureInner> for JsTexture {
    fn from(js_texture_inner: &JsTextureInner) -> Self {
        Self(js_texture_inner.to_owned())
    }
}
