use wrend::Id;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub enum TextureId {
    #[default]
    PrevRenderA,
    PrevRenderB,
    SrcVideo,
}

impl Id for TextureId {}

impl TextureId {
    pub fn location(&self) -> u32 {
        match self {
            TextureId::PrevRenderA => 0,
            TextureId::PrevRenderB => 1,
            TextureId::SrcVideo => 2,
        }
    }
}
