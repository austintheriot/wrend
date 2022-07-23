use wrend::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TextureId {
    WhiteNoise,
    SimplexNoise,
}

impl Id for TextureId {}

impl Default for TextureId {
    fn default() -> Self {
        Self::WhiteNoise
    }
}
