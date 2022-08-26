use crate::{Id, TextureJs, TextureJsInner};
use std::fmt::Debug;
use std::hash::Hash;
use wasm_bindgen::JsValue;
use web_sys::WebGlTexture;

#[derive(Clone)]
pub struct Texture<TextureId: Id> {
    texture_id: TextureId,
    webgl_texture: WebGlTexture,
}

impl<TextureId: Id> Texture<TextureId> {
    // @todo move into builder pattern ?
    pub fn new(texture_id: TextureId, webgl_texture: WebGlTexture) -> Self {
        Self {
            texture_id,
            webgl_texture,
        }
    }

    pub fn texture_id(&self) -> &TextureId {
        &self.texture_id
    }

    pub fn webgl_texture(&self) -> &WebGlTexture {
        &self.webgl_texture
    }
}

impl<TextureId: Id> Debug for Texture<TextureId> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Texture")
            .field("texture_id", &self.texture_id)
            .field("webgl_texture", &self.webgl_texture)
            .finish()
    }
}
impl<TextureId: Id> Hash for Texture<TextureId> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.texture_id.hash(state);
    }
}

impl<TextureId: Id> PartialEq for Texture<TextureId> {
    fn eq(&self, other: &Self) -> bool {
        self.texture_id == other.texture_id && self.webgl_texture == other.webgl_texture
    }
}

impl<TextureId: Id> Eq for Texture<TextureId> {}

impl From<TextureJsInner> for JsValue {
    fn from(texture: TextureJsInner) -> Self {
        let js_texture: TextureJs = texture.into();
        js_texture.into()
    }
}
