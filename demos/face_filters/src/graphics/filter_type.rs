#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FilterType {
    #[default]
    Unfiltered,
    Grayscale
}