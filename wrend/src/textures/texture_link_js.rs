use std::ops::{Deref, DerefMut};

use js_sys::Function;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{WebGl2RenderingContext, WebGlTexture};

use crate::TextureLink;

pub type TextureLinkJsInner = TextureLink<String>;

#[wasm_bindgen(js_name = TextureLink)]
pub struct TextureLinkJs(TextureLinkJsInner);

#[wasm_bindgen(js_class = TextureLink)]
impl TextureLinkJs {
    #[wasm_bindgen(constructor)]
    pub fn new(texture_id: String, create_texture_callback: Function) -> Self {
        Self(TextureLinkJsInner::new(texture_id, create_texture_callback))
    }

    #[wasm_bindgen(js_name = textureId)]
    pub fn texture_id(&self) -> String {
        self.deref().texture_id().to_owned()
    }

    #[wasm_bindgen(js_name = createTexture)]
    pub fn create_texture(&self, gl: WebGl2RenderingContext, now: f64) -> WebGlTexture {
        self.deref().create_texture(gl, now)
    }
}

impl TextureLinkJs {
    pub fn inner(self) -> TextureLinkJsInner {
        self.0
    }
}

impl Deref for TextureLinkJs {
    type Target = TextureLinkJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TextureLinkJs {
    fn deref_mut(&mut self) -> &mut TextureLinkJsInner {
        &mut self.0
    }
}

impl From<TextureLinkJs> for TextureLinkJsInner {
    fn from(buffer_link_js: TextureLinkJs) -> Self {
        buffer_link_js.inner()
    }
}
