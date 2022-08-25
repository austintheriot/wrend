use std::ops::{Deref, DerefMut};

use crate::Texture;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::WebGlTexture;

pub type JsTextureInner = Texture<String>;

#[wasm_bindgen(js_name = Texture)]
pub struct JsTexture(JsTextureInner);

#[wasm_bindgen(js_class = Texture)]
impl JsTexture {
    pub fn texture_id(&self) -> String {
        self.deref().texture_id().to_owned()
    }

    pub fn webgl_texture(&self) -> WebGlTexture {
        self.deref().webgl_texture().to_owned()
    }
}

impl Deref for JsTexture {
    type Target = JsTextureInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JsTexture {
    fn deref_mut(&mut self) -> &mut JsTextureInner {
        &mut self.0
    }
}

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
