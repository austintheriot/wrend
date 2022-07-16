use super::id::Id;
use super::texture_create_context::TextureCreateContext;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, WebGlTexture};

pub type TextureCreateCallback<UserCtx> = Rc<dyn Fn(TextureCreateContext<UserCtx>) -> WebGlTexture>;

#[derive(Clone)]
pub struct TextureLink<TextureId: Id, UserCtx> {
    texture_id: TextureId,
    create_texture_callback: TextureCreateCallback<UserCtx>,
}

impl<TextureId: Id, UserCtx> TextureLink<TextureId, UserCtx> {
    pub fn new(
        texture_id: TextureId,
        create_texture_callback: TextureCreateCallback<UserCtx>,
    ) -> Self {
        Self {
            texture_id,
            create_texture_callback,
        }
    }

    pub fn texture_id(&self) -> &TextureId {
        &self.texture_id
    }

    pub fn create_texture(
        &self,
        gl: &WebGl2RenderingContext,
        now: f64,
        user_ctx: Option<&UserCtx>,
    ) -> WebGlTexture {
        let texture_create_context = TextureCreateContext::new(gl, now, user_ctx);
        (self.create_texture_callback)(texture_create_context)
    }
}

impl<TextureId: Id, UserCtx> Debug for TextureLink<TextureId, UserCtx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextureLink")
            .field("texture_id", &self.texture_id)
            .finish()
    }
}

impl<TextureId: Id, UserCtx> Hash for TextureLink<TextureId, UserCtx> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.texture_id.hash(state);
    }
}

impl<TextureId: Id, UserCtx> PartialEq for TextureLink<TextureId, UserCtx> {
    fn eq(&self, other: &Self) -> bool {
        self.texture_id == other.texture_id
    }
}

impl<TextureId: Id, UserCtx> Eq for TextureLink<TextureId, UserCtx> {}