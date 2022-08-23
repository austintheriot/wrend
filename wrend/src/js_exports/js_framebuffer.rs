use crate::Framebuffer;
use wasm_bindgen::prelude::wasm_bindgen;

pub type JsFramebufferInner = Framebuffer<String>;

#[wasm_bindgen]
pub struct JsFramebuffer(JsFramebufferInner);

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
