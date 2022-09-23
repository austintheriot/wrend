use std::fmt::Display;

use strum::EnumIter;

use super::{FragmentShaderId, ProgramId};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum FilterType {
    #[default]
    Unfiltered,
    Split,
}

impl FilterType {
    /// Gets the associated ProgramId for the FilterType
    pub fn program_id(&self) -> ProgramId {
        match self {
            FilterType::Unfiltered => ProgramId::FilterUnfiltered,
            FilterType::Split => ProgramId::FilterSplit,
        }
    }

    /// Gets the associated FragmentShaderId for the FilterType
    pub fn fragment_shader_id(&self) -> FragmentShaderId {
        match self {
            FilterType::Unfiltered => FragmentShaderId::FilterUnfiltered,
            FilterType::Split => FragmentShaderId::FilterSplit,
        }
    }
}

impl Display for FilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterType::Unfiltered => write!(f, "Unfiltered"),
            FilterType::Split => write!(f, "Split"),
        }
    }
}
