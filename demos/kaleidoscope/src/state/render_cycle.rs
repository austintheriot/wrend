use crate::graphics::{FramebufferId, TextureId};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderCycle {
    #[default]
    A,
    B,
}

impl RenderCycle {
    pub fn next(&self) -> Self {
        match self {
            RenderCycle::A => RenderCycle::B,
            RenderCycle::B => RenderCycle::A,
        }
    }

    pub fn texture_id(&self) -> TextureId {
        match self {
            RenderCycle::A => TextureId::PrevRenderA,
            RenderCycle::B => TextureId::PrevRenderB,
        }
    }

    pub fn framebuffer_id(&self) -> FramebufferId {
        match self {
            RenderCycle::A => FramebufferId::PrevRenderA,
            RenderCycle::B => FramebufferId::PrevRenderB,
        }
    }
}
