use wrend::Id;

use super::texture_id_number::TextureIdNumber;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TextureId {
    WhiteNoise,
    PerlinNoise,
    PrevFrameA,
    PrevFrameB,
}

impl Id for TextureId {}

impl Default for TextureId {
    fn default() -> Self {
        Self::WhiteNoise
    }
}

impl TextureIdNumber for TextureId {
    fn num(&self ) -> u32 {
        match self {
            TextureId::WhiteNoise => 0,
            TextureId::PerlinNoise => 1,
            TextureId::PrevFrameA => 2,
            TextureId::PrevFrameB => 3,
        } 
    }
}