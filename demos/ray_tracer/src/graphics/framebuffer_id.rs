use wrend::Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FramebufferId {
    RenderA,
    RenderB,
}

impl Default for FramebufferId {
    fn default() -> Self {
        Self::RenderA
    }
}

impl Id for FramebufferId {}
