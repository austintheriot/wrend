use crate::{FramebufferCreateContext, IntoJsWrapper};
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{WebGl2RenderingContext, WebGlTexture};

pub type BufferCreateContextJsInner = FramebufferCreateContext;

#[wasm_bindgen(js_name = FramebufferCreateContext)]
pub struct FramebufferCreateContextJs(BufferCreateContextJsInner);

#[wasm_bindgen(js_class = FramebufferCreateContext)]
impl FramebufferCreateContextJs {
    #[wasm_bindgen(constructor)]
    pub fn new(gl: WebGl2RenderingContext, now: f64, webgl_texture: Option<WebGlTexture>) -> Self {
        Self(FramebufferCreateContext::new(gl, now, webgl_texture))
    }

    pub fn gl(&self) -> WebGl2RenderingContext {
        self.deref().gl().to_owned()
    }

    pub fn now(&self) -> f64 {
        self.deref().now()
    }

    pub fn webgl_texture(&self) -> Option<WebGlTexture> {
        self.deref().webgl_texture().to_owned()
    }
}

impl FramebufferCreateContextJs {
    pub fn inner(self) -> BufferCreateContextJsInner {
        self.0
    }
}

impl Deref for FramebufferCreateContextJs {
    type Target = BufferCreateContextJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FramebufferCreateContextJs {
    fn deref_mut(&mut self) -> &mut BufferCreateContextJsInner {
        &mut self.0
    }
}

impl From<FramebufferCreateContext> for FramebufferCreateContextJs {
    fn from(attribute_create_context: FramebufferCreateContext) -> Self {
        FramebufferCreateContextJs(attribute_create_context)
    }
}

impl IntoJsWrapper for FramebufferCreateContext {
    type Result = FramebufferCreateContextJs;

    fn into_js_wrapper(self) -> Self::Result {
        self.into()
    }
}

impl From<&FramebufferCreateContext> for FramebufferCreateContextJs {
    fn from(attribute_create_context: &FramebufferCreateContext) -> Self {
        FramebufferCreateContextJs(attribute_create_context.to_owned())
    }
}

impl IntoJsWrapper for &FramebufferCreateContext {
    type Result = FramebufferCreateContextJs;

    fn into_js_wrapper(self) -> Self::Result {
        self.into()
    }
}
