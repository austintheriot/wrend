use webgl::renderer::id::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FramebufferId {
    A,
    B,
}

impl Id for FramebufferId {}

impl Default for FramebufferId {
    fn default() -> Self {
        Self::A
    }
}

