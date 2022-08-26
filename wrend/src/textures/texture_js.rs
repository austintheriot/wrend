use std::ops::{Deref, DerefMut};

use crate::Texture;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::WebGlTexture;

pub type TextureJsInner = Texture<String>;

#[wasm_bindgen(js_name = Texture)]
pub struct TextureJs(TextureJsInner);

#[wasm_bindgen(js_class = Texture)]
impl TextureJs {
    pub fn texture_id(&self) -> String {
        self.deref().texture_id().to_owned()
    }

    pub fn webgl_texture(&self) -> WebGlTexture {
        self.deref().webgl_texture().to_owned()
    }
}

impl Deref for TextureJs {
    type Target = TextureJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TextureJs {
    fn deref_mut(&mut self) -> &mut TextureJsInner {
        &mut self.0
    }
}

impl From<TextureJsInner> for TextureJs {
    fn from(js_texture_inner: TextureJsInner) -> Self {
        Self(js_texture_inner)
    }
}

impl From<&TextureJsInner> for TextureJs {
    fn from(js_texture_inner: &TextureJsInner) -> Self {
        Self(js_texture_inner.to_owned())
    }
}
