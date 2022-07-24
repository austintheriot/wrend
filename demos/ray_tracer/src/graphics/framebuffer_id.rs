use wrend::Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FramebufferId {
    PrevRender,
    AveragedRenderA,
    AveragedRenderB,
}

impl Default for FramebufferId {
    fn default() -> Self {
        Self::PrevRender
    }
}

impl Id for FramebufferId {}
