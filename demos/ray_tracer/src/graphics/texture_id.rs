use wrend::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TextureId {
    RenderA,
    RenderB,
}

impl Id for TextureId {}

impl Default for TextureId {
    fn default() -> Self {
        Self::RenderA
    }
}

impl TextureId {
    pub fn location(&self) -> u32 {
        match self {
            TextureId::RenderA => 0,
            TextureId::RenderB => 1,
        }
    }
}