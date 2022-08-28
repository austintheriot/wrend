use std::ops::{Deref, DerefMut};

use js_sys::{Function, Object};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{WebGl2RenderingContext, WebGlFramebuffer, WebGlTexture};

use crate::FramebufferLink;

pub type FramebufferLinkJsInner = FramebufferLink<String, Object, String>;

#[wasm_bindgen(js_name = FramebufferLink)]
#[derive(Clone)]
pub struct FramebufferLinkJs(FramebufferLinkJsInner);

#[wasm_bindgen(js_class = FramebufferLink)]
impl FramebufferLinkJs {
    #[wasm_bindgen(constructor)]
    pub fn new(
        framebuffer_id: String,
        framebuffer_create_callback: Function,
        texture_id: Option<String>,
    ) -> Self {
        Self(FramebufferLinkJsInner::new(
            framebuffer_id,
            framebuffer_create_callback,
            texture_id,
        ))
    }

    pub fn framebuffer_id(&self) -> String {
        self.deref().framebuffer_id().to_owned()
    }

    pub fn texture_id(&self) -> Option<String> {
        self.deref().texture_id()
    }

    pub fn create_framebuffer(
        &self,
        gl: WebGl2RenderingContext,
        now: f64,
        texture: Option<WebGlTexture>,
        user_ctx: Option<Object>,
    ) -> WebGlFramebuffer {
        self.deref().create_framebuffer(gl, now, texture, user_ctx)
    }
}

impl FramebufferLinkJs {
    pub fn inner(self) -> FramebufferLinkJsInner {
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
