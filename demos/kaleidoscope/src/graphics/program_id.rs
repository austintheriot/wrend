use wrend::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ProgramId {
    GenerateCircleGradient,
    GenerateLinearGradient,
    FilterUnfiltered,
    FilterSplit,
    FilterTriangleReflection,
}

impl Id for ProgramId {}

impl Default for ProgramId {
    fn default() -> Self {
        Self::GenerateCircleGradient
    }
}
