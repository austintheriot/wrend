use std::fmt::Display;

use strum::EnumIter;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum FilterType {
    #[default]
    Unfiltered,
    Grayscale,
    Invert,
    Wavy,
}

impl Display for FilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterType::Unfiltered => write!(f, "Unfiltered"),
            FilterType::Grayscale => write!(f, "Grayscale"),
            FilterType::Invert => write!(f, "Invert"),
            FilterType::Wavy => write!(f, "Wavy"),
        }
    }
}
