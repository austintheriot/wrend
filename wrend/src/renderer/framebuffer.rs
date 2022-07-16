use super::id::Id;
use std::fmt::Debug;
use std::hash::Hash;
use web_sys::WebGlFramebuffer;

#[derive(Clone)]
pub struct Framebuffer<ProgramId: Id, FramebufferId: Id> {
    program_id: ProgramId,
    framebuffer_id: FramebufferId,
    webgl_framebuffer: WebGlFramebuffer,
}

impl<ProgramId: Id, FramebufferId: Id> Framebuffer<ProgramId, FramebufferId> {
    // @todo move into builder pattern ?
    pub fn new(program_id: ProgramId, framebuffer_id: FramebufferId, webgl_framebuffer: WebGlFramebuffer) -> Self {
        Self {
            program_id,
            framebuffer_id,
            webgl_framebuffer,
        }
    }

    pub fn program_id(&self) -> &ProgramId {
        &self.program_id
    }

    pub fn framebuffer_id(&self) -> &FramebufferId {
        &self.framebuffer_id
    }

    pub fn webgl_framebuffer(&self) -> &WebGlFramebuffer {
        &self.webgl_framebuffer
    }
}

impl<ProgramId: Id, FramebufferId: Id> Debug for Framebuffer<ProgramId, FramebufferId> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Framebuffer")
            .field("program_id", &self.program_id)
            .field("framebuffer_id", &self.framebuffer_id)
            .field("webgl_framebuffer", &self.webgl_framebuffer)
            .finish()
    }
}
impl<ProgramId: Id, FramebufferId: Id> Hash for Framebuffer<ProgramId, FramebufferId> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.framebuffer_id.hash(state);
    }
}

impl<ProgramId: Id, FramebufferId: Id> PartialEq for Framebuffer<ProgramId, FramebufferId> {
    fn eq(&self, other: &Self) -> bool {
        self.program_id == other.program_id
            && self.framebuffer_id == other.framebuffer_id
            && self.webgl_framebuffer == other.webgl_framebuffer
    }
}

impl<ProgramId: Id, FramebufferId: Id> Eq for Framebuffer<ProgramId, FramebufferId> {}
