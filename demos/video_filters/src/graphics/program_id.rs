use wrend::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ProgramId {
    Unfiltered,
    Grayscale,
    Invert,
    Wavy,
}

impl Id for ProgramId {}

impl Default for ProgramId {
    fn default() -> Self {
        Self::Unfiltered
    }
}
