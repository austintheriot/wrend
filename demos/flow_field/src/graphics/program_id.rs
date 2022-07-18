use wrend::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ProgramId {
    PerlinNoise,
    FlowField,
    PassThrough,
}

impl Id for ProgramId {}

impl Default for ProgramId {
    fn default() -> Self {
        Self::PassThrough
    }
}
