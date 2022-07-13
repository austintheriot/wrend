use super::id::Id;
use std::fmt::Debug;
use std::hash::Hash;
use web_sys::WebGlTexture;

#[derive(Clone)]
pub struct Texture<ProgramId: Id, TextureId: Id> {
    program_id: ProgramId,
    texture_id: TextureId,
    webgl_texture: WebGlTexture,
}

impl<ProgramId: Id, TextureId: Id> Texture<ProgramId, TextureId> {
    // @todo move into builder pattern ?
    pub fn new(program_id: ProgramId, texture_id: TextureId, webgl_texture: WebGlTexture) -> Self {
        Self {
            program_id,
            texture_id,
            webgl_texture,
        }
    }

    pub fn program_id(&self) -> &ProgramId {
        &self.program_id
    }

    pub fn texture_id(&self) -> &TextureId {
        &self.texture_id
    }

    pub fn webgl_texture(&self) -> &WebGlTexture {
        &self.webgl_texture
    }
}

impl<ProgramId: Id, TextureId: Id> Debug for Texture<ProgramId, TextureId> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Texture")
            .field("program_id", &self.program_id)
            .field("texture_id", &self.texture_id)
            .field("webgl_texture", &self.webgl_texture)
            .finish()
    }
}
impl<ProgramId: Id, TextureId: Id> Hash for Texture<ProgramId, TextureId> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.texture_id.hash(state);
    }
}

impl<ProgramId: Id, TextureId: Id> PartialEq for Texture<ProgramId, TextureId> {
    fn eq(&self, other: &Self) -> bool {
        self.program_id == other.program_id
            && self.texture_id == other.texture_id
            && self.webgl_texture == other.webgl_texture
    }
}

impl<ProgramId: Id, TextureId: Id> Eq for Texture<ProgramId, TextureId> {}
