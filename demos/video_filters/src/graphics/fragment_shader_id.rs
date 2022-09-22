use wrend::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FragmentShaderId {
    Unfiltered,
    Grayscale,
    Invert
}

impl Id for FragmentShaderId {}

impl Default for FragmentShaderId {
    fn default() -> Self {
        Self::Unfiltered
    }
}
