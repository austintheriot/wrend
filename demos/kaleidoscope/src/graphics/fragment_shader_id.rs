use wrend::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FragmentShaderId {
    GenerateCircleGradient,
    GenerateLinearGradient,
    FilterUnfiltered,
    FilterSplit,
    FilterTriangleReflection
}

impl Id for FragmentShaderId {}

impl Default for FragmentShaderId {
    fn default() -> Self {
        Self::FilterUnfiltered
    }
}
