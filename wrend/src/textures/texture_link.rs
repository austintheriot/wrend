use super::texture_create_context::TextureCreateContext;
use crate::{Id, TextureCreateCallback};
use std::fmt::Debug;
use std::hash::Hash;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlTexture};

#[derive(Clone)]
pub struct TextureLink<TextureId: Id> {
    texture_id: TextureId,
    create_texture_callback: TextureCreateCallback,
}

impl<TextureId: Id> TextureLink<TextureId> {
    pub fn new(
        texture_id: TextureId,
        create_texture_callback: impl Into<TextureCreateCallback>,
    ) -> Self {
        Self {
            texture_id,
            create_texture_callback: create_texture_callback.into(),
        }
    }

    pub fn texture_id(&self) -> &TextureId {
        &self.texture_id
    }

    pub fn create_texture(
        &self,
        gl: WebGl2RenderingContext,
        now: f64,
        canvas: HtmlCanvasElement,
    ) -> WebGlTexture {
        let texture_create_context = TextureCreateContext::new(gl, now, canvas);
        self.create_texture_callback
            .call_with_into_js_arg_and_return(&texture_create_context)
    }
}

impl<TextureId: Id> Debug for TextureLink<TextureId> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextureLink")
            .field("texture_id", &self.texture_id)
            .field("create_texture_callback", &self.create_texture_callback)
            .finish()
    }
}

impl<TextureId: Id> Hash for TextureLink<TextureId> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.texture_id.hash(state);
        self.create_texture_callback.hash(state);
    }
}

impl<TextureId: Id> PartialEq for TextureLink<TextureId> {
    fn eq(&self, other: &Self) -> bool {
        self.texture_id == other.texture_id
            && self.create_texture_callback == other.create_texture_callback
    }
}

impl<TextureId: Id> Eq for TextureLink<TextureId> {}
