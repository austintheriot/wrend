use std::ops::{Deref, DerefMut};

use js_sys::{Function, Object};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

use crate::BufferLink;

pub type JsBufferLinkInner = BufferLink<String, Object>;

#[wasm_bindgen(js_name = BufferLink)]
pub struct JsBufferLink(JsBufferLinkInner);

#[wasm_bindgen(js_class = BufferLink)]
impl JsBufferLink {
    #[wasm_bindgen(constructor)]
    pub fn new(buffer_id: String, buffer_create_callback: Function) -> Self {
        Self(JsBufferLinkInner::new(buffer_id, buffer_create_callback))
    }

    pub fn buffer_id(&self) -> String {
        self.deref().buffer_id().to_owned()
    }

    pub fn create_buffer(
        &self,
        gl: WebGl2RenderingContext,
        now: f64,
        user_ctx: Option<Object>,
    ) -> WebGlBuffer {
        self.deref().create_buffer(gl, now, user_ctx)
    }
}

impl JsBufferLink {
    pub fn inner(self) -> JsBufferLinkInner {
        self.0
    }
}

impl Deref for JsBufferLink {
    type Target = JsBufferLinkInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JsBufferLink {
    fn deref_mut(&mut self) -> &mut JsBufferLinkInner {
        &mut self.0
    }
}
