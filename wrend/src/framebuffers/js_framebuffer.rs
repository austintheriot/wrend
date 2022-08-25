use crate::Framebuffer;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::WebGlFramebuffer;

pub type JsFramebufferInner = Framebuffer<String>;

#[wasm_bindgen(js_name = Framebuffer)]
pub struct JsFramebuffer(JsFramebufferInner);

#[wasm_bindgen(js_name = Framebuffer)]
impl JsFramebuffer {
    pub fn framebuffer_id(&self) -> String {
        self.deref().framebuffer_id().to_owned()
    }

    pub fn webgl_framebuffer(&self) -> WebGlFramebuffer {
        self.deref().webgl_framebuffer().to_owned()
    }
}

impl From<JsFramebufferInner> for JsFramebuffer {
    fn from(js_framebuffer_inner: JsFramebufferInner) -> Self {
        Self(js_framebuffer_inner)
    }
}

impl From<&JsFramebufferInner> for JsFramebuffer {
    fn from(js_framebuffer_inner: &JsFramebufferInner) -> Self {
        Self(js_framebuffer_inner.to_owned())
    }
}

impl Deref for JsFramebuffer {
    type Target = JsFramebufferInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JsFramebuffer {
    fn deref_mut(&mut self) -> &mut JsFramebufferInner {
        &mut self.0
    }
}
