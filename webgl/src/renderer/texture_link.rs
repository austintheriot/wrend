use super::id::Id;
use super::texture_create_context::TextureCreateContext;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, WebGlTexture};

pub type CreateTextureCallback<UserCtx> = Rc<dyn Fn(TextureCreateContext<UserCtx>) -> WebGlTexture>;

#[derive(Clone)]
pub struct TextureLink<ProgramId, TextureId, UserCtx>
where
    ProgramId: Id,
    TextureId: Id,
{
    program_id: ProgramId,
    texture_id: TextureId,
    create_texture_callback: CreateTextureCallback<UserCtx>,
}

impl<ProgramId, TextureId, UserCtx> TextureLink<ProgramId, TextureId, UserCtx>
where
    ProgramId: Id,
    TextureId: Id,
{
    pub fn new(
        program_id: ProgramId,
        texture_id: TextureId,
        create_texture_callback: CreateTextureCallback<UserCtx>,
    ) -> Self {
        Self {
            program_id,
            texture_id,
            create_texture_callback,
        }
    }

    pub fn program_id(&self) -> &ProgramId {
        &self.program_id
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

impl<ProgramId, TextureId, UserCtx> Debug for TextureLink<ProgramId, TextureId, UserCtx>
where
    ProgramId: Id,
    TextureId: Id,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextureLink")
            .field("program_id", &self.program_id)
            .field("update_callback", &"[not shown]")
            .field("should_update_callback", &"[not shown]")
            .finish()
    }
}

impl<ProgramId, TextureId, UserCtx> Hash for TextureLink<ProgramId, TextureId, UserCtx>
where
    ProgramId: Id,
    TextureId: Id,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.program_id.hash(state);
        self.texture_id.hash(state);
    }
}

impl<ProgramId, TextureId, UserCtx> PartialEq for TextureLink<ProgramId, TextureId, UserCtx>
where
    ProgramId: Id,
    TextureId: Id,
{
    fn eq(&self, other: &Self) -> bool {
        self.program_id == other.program_id && self.texture_id == other.texture_id
    }
}

impl<ProgramId, TextureId, UserCtx> Eq for TextureLink<ProgramId, TextureId, UserCtx>
where
    ProgramId: Id,
    TextureId: Id,
{
}
