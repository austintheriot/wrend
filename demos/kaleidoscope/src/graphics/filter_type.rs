use std::fmt::Display;

use strum::EnumIter;

use super::{FragmentShaderId, ProgramId};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum FilterType {
    #[default]
    Unfiltered,
    Split,
    TriangleReflection,
    OffsetFragments,
    MovingFragments,
}

impl FilterType {
    /// Gets the associated ProgramId for the FilterType
    pub fn program_id(&self) -> ProgramId {
        match self {
            FilterType::Unfiltered => ProgramId::FilterUnfiltered,
            FilterType::Split => ProgramId::FilterSplit,
            FilterType::TriangleReflection => ProgramId::FilterTriangleReflection,
            FilterType::OffsetFragments => ProgramId::FilterOffsetFragments,
            FilterType::MovingFragments => ProgramId::FilterMovingFragments,
        }
    }

    /// Gets the associated FragmentShaderId for the FilterType
    pub fn fragment_shader_id(&self) -> FragmentShaderId {
        match self {
            FilterType::Unfiltered => FragmentShaderId::FilterUnfiltered,
            FilterType::Split => FragmentShaderId::FilterSplit,
            FilterType::TriangleReflection => FragmentShaderId::FilterTriangleReflection,
            FilterType::OffsetFragments => FragmentShaderId::FilterOffsetFragments,
            FilterType::MovingFragments => FragmentShaderId::FilterMovingFragments,
        }
    }
}

impl Display for FilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterType::Unfiltered => write!(f, "Unfiltered"),
            FilterType::Split => write!(f, "Split"),
            FilterType::TriangleReflection => write!(f, "Triangle Reflection"),
            FilterType::OffsetFragments => write!(f, "Offset Fragments"),
            FilterType::MovingFragments => write!(f, "Moving Fragments"),
        }
    }
}
