use wrend::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ProgramId {
    RayTracer,
    AverageRenders,
    PassThrough,
}

impl Id for ProgramId {}

impl Default for ProgramId {
    fn default() -> Self {
        Self::PassThrough
    }
}
