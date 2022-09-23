use std::fmt::Display;

use strum::EnumIter;

/// This represents the possible initially generated source textures we can work with
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum GenerationType {
    #[default]
    CircleGradient,
}

impl Display for GenerationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerationType::CircleGradient => write!(f, "CircleGradient"),
        }
    }
}
