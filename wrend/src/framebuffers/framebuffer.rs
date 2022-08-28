use crate::{FramebufferJs, FramebufferJsInner, Id};
use std::fmt::Debug;
use std::hash::Hash;
use wasm_bindgen::JsValue;
use web_sys::WebGlFramebuffer;

#[derive(Clone)]
pub struct Framebuffer<FramebufferId: Id> {
    framebuffer_id: FramebufferId,
    webgl_framebuffer: WebGlFramebuffer,
}

impl<FramebufferId: Id> Framebuffer<FramebufferId> {
    // @todo move into builder pattern ?
    pub fn new(framebuffer_id: FramebufferId, webgl_framebuffer: WebGlFramebuffer) -> Self {
        Self {
            framebuffer_id,
            webgl_framebuffer,
        }
    }

    pub fn framebuffer_id(&self) -> &FramebufferId {
        &self.framebuffer_id
    }

    pub fn webgl_framebuffer(&self) -> &WebGlFramebuffer {
        &self.webgl_framebuffer
    }
}

impl<FramebufferId: Id> Debug for Framebuffer<FramebufferId> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Framebuffer")
            .field("framebuffer_id", &self.framebuffer_id)
            .field("webgl_framebuffer", &self.webgl_framebuffer)
            .finish()
    }
}
impl<FramebufferId: Id> Hash for Framebuffer<FramebufferId> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.framebuffer_id.hash(state);
    }
}

impl<FramebufferId: Id> PartialEq for Framebuffer<FramebufferId> {
    fn eq(&self, other: &Self) -> bool {
        self.framebuffer_id == other.framebuffer_id
            && self.webgl_framebuffer == other.webgl_framebuffer
    }
}

impl<FramebufferId: Id> Eq for Framebuffer<FramebufferId> {}

impl From<FramebufferJsInner> for JsValue {
    fn from(framebuffer: FramebufferJsInner) -> Self {
        let js_framebuffer: FramebufferJs = framebuffer.into();
        js_framebuffer.into()
    }
}
