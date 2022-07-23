use wrend::Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FramebufferId {
    SimplexNoise,
}

impl Default for FramebufferId {
    fn default() -> Self {
        Self::SimplexNoise
    }
}

impl Id for FramebufferId {}
