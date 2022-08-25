use crate::Texture;
use wasm_bindgen::prelude::wasm_bindgen;

pub type JsTextureInner = Texture<String>;

#[wasm_bindgen(js_name = Texture)]
pub struct JsTexture(JsTextureInner);

#[wasm_bindgen(js_class = Texture)]
impl JsTexture {}


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
