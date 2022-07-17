use wrend::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ProgramId {
    FlowField,
    PassThrough,
}

impl Id for ProgramId {}

impl Default for ProgramId {
    fn default() -> Self {
        Self::FlowField
    }
}
