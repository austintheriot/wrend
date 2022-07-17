use wrend::renderer::id::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FragmentShaderId {
    GameOfLife,
    PassThrough,
}

impl Id for FragmentShaderId {}

impl Default for FragmentShaderId {
    fn default() -> Self {
        Self::PassThrough
    }
}
