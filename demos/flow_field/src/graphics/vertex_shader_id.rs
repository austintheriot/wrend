use wrend::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum VertexShaderId {
    Quad,
}

impl Id for VertexShaderId {}

impl Default for VertexShaderId {
    fn default() -> Self {
        Self::Quad
    }
}
