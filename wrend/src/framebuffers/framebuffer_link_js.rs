use std::ops::{Deref, DerefMut};

use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{WebGl2RenderingContext, WebGlFramebuffer, WebGlTexture};

use crate::{FramebufferCreateCallbackJs, FramebufferLink};

pub type FramebufferLinkJsInner = FramebufferLink<String, String>;

#[wasm_bindgen(inspectable, js_name = FramebufferLink)]
#[derive(Clone)]
pub struct FramebufferLinkJs(FramebufferLinkJsInner);

#[wasm_bindgen(js_class = FramebufferLink)]
impl FramebufferLinkJs {
    #[wasm_bindgen(constructor)]
    pub fn new(
        framebuffer_id: String,
        framebuffer_create_callback: FramebufferCreateCallbackJs,
        texture_id: Option<String>,
    ) -> Self {
        Self(FramebufferLinkJsInner::new(
            framebuffer_id,
            framebuffer_create_callback,
            texture_id,
        ))
    }

    #[wasm_bindgen(js_name = framebufferId)]
    pub fn framebuffer_id(&self) -> String {
        self.deref().framebuffer_id().to_owned()
    }

    #[wasm_bindgen(js_name = textureId)]
    pub fn texture_id(&self) -> Option<String> {
        self.deref().texture_id()
    }

    #[wasm_bindgen(js_name = createFramebuffer)]
    pub fn create_framebuffer(
        &self,
        gl: WebGl2RenderingContext,
        now: f64,
        texture: Option<WebGlTexture>,
    ) -> WebGlFramebuffer {
        self.deref().create_framebuffer(gl, now, texture)
    }
}

impl FramebufferLinkJs {
    pub fn into_inner(self) -> FramebufferLinkJsInner {
        self.0
    }
}

impl Deref for FramebufferLinkJs {
    type Target = FramebufferLinkJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FramebufferLinkJs {
    fn deref_mut(&mut self) -> &mut FramebufferLinkJsInner {
        &mut self.0
    }
}

impl From<FramebufferLinkJs> for FramebufferLinkJsInner {
    fn from(framebuffer_link_js: FramebufferLinkJs) -> Self {
        framebuffer_link_js.into_inner()
    }
}
