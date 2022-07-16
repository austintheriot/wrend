use wrend::renderer::id::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ShaderId {
    Vertex,
    FlowField,
    PassThrough,
}

impl Id for ShaderId {}

impl Default for ShaderId {
    fn default() -> Self {
        Self::Vertex
    }
}
