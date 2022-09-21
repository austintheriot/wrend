use wrend::Id;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FramebufferId {
    #[default]
    PrevRenderA,
    PrevRenderB,
}

impl Id for FramebufferId {}
