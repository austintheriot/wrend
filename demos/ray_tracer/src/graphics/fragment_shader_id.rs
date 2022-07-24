use wrend::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FragmentShaderId {
    PassThrough,
    RayTracer,
}

impl Id for FragmentShaderId {}

impl Default for FragmentShaderId {
    fn default() -> Self {
        Self::PassThrough
    }
}
