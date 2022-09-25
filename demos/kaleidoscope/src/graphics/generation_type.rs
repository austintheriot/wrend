use std::fmt::Display;

use strum::EnumIter;

use super::{FragmentShaderId, ProgramId};

/// This represents the possible initially generated source textures we can work with
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum GenerationType {
    #[default]
    CircleGradient,
    LinearGradient,
    VideoInput,
}

impl GenerationType {
    /// Gets the associated ProgramId for the GenerationType
    pub fn program_id(&self) -> ProgramId {
        match self {
            GenerationType::CircleGradient => ProgramId::GenerateCircleGradient,
            GenerationType::LinearGradient => ProgramId::GenerateLinearGradient,
            GenerationType::VideoInput => ProgramId::GenerateVideoInput,
        }
    }

    /// Gets the associated FragmentShaderId for the GenerationType
    pub fn fragment_shader_id(&self) -> FragmentShaderId {
        match self {
            GenerationType::CircleGradient => FragmentShaderId::GenerateCircleGradient,
            GenerationType::LinearGradient => FragmentShaderId::GenerateLinearGradient,
            GenerationType::VideoInput => FragmentShaderId::GenerateVideoInput,
        }
    }
}

impl Display for GenerationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerationType::CircleGradient => write!(f, "Circle Gradient"),
            GenerationType::LinearGradient => write!(f, "Linear Gradient"),
            GenerationType::VideoInput => write!(f, "Video Input"),
        }
    }
}
