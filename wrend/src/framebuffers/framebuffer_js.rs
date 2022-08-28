use crate::Framebuffer;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::WebGlFramebuffer;

pub type FramebufferJsInner = Framebuffer<String>;

#[wasm_bindgen(js_name = Framebuffer)]
pub struct FramebufferJs(FramebufferJsInner);

#[wasm_bindgen(js_class = Framebuffer)]
impl FramebufferJs {
    pub fn framebuffer_id(&self) -> String {
        self.deref().framebuffer_id().to_owned()
    }

    pub fn webgl_framebuffer(&self) -> WebGlFramebuffer {
        self.deref().webgl_framebuffer().to_owned()
    }
}

impl From<FramebufferJsInner> for FramebufferJs {
    fn from(js_framebuffer_inner: FramebufferJsInner) -> Self {
        Self(js_framebuffer_inner)
    }
}

impl From<&FramebufferJsInner> for FramebufferJs {
    fn from(js_framebuffer_inner: &FramebufferJsInner) -> Self {
        Self(js_framebuffer_inner.to_owned())
    }
}

impl Deref for FramebufferJs {
    type Target = FramebufferJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FramebufferJs {
    fn deref_mut(&mut self) -> &mut FramebufferJsInner {
        &mut self.0
    }
}
