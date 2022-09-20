use std::default;

use wrend::Id;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub enum TextureId {
    #[default]
    PrevRenderA,
    PrevRenderB,
}

impl Id for TextureId {}

impl TextureId {
    pub fn location(&self) -> u32 {
        match self {
            TextureId::PrevRenderA => 0,
            TextureId::PrevRenderB => 1,
        }
    }
}
