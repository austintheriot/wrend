use webgl::renderer::id::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TextureId {
    A,
    B,
}

impl Id for TextureId {}

impl Default for TextureId {
    fn default() -> Self {
        Self::A
    }
}

