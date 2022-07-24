use wrend::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TextureId {
    PrevRender,
    AveragedRenderA,
    AveragedRenderB,
}

impl Id for TextureId {}

impl Default for TextureId {
    fn default() -> Self {
        Self::PrevRender
    }
}

impl TextureId {
    pub fn location(&self) -> u32 {
        match self {
            TextureId::PrevRender => 0,
            TextureId::AveragedRenderA => 1,
            TextureId::AveragedRenderB => 1,
        }
    }
}