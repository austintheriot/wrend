use crate::{BufferCreateContext, IntoJsWrapper};
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::WebGl2RenderingContext;

pub type BufferCreateContextJsInner = BufferCreateContext;

#[wasm_bindgen(inspectable, js_name = BufferCreateContext)]
pub struct BufferCreateContextJs(BufferCreateContextJsInner);

#[wasm_bindgen(js_class = BufferCreateContext)]
impl BufferCreateContextJs {
    pub fn gl(&self) -> WebGl2RenderingContext {
        self.deref().gl().to_owned()
    }

    pub fn now(&self) -> f64 {
        self.deref().now()
    }
}

impl BufferCreateContextJs {
    pub fn into_inner(self) -> BufferCreateContextJsInner {
        self.0
    }
}

impl Deref for BufferCreateContextJs {
    type Target = BufferCreateContextJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BufferCreateContextJs {
    fn deref_mut(&mut self) -> &mut BufferCreateContextJsInner {
        &mut self.0
    }
}

impl From<BufferCreateContext> for BufferCreateContextJs {
    fn from(attribute_create_context: BufferCreateContext) -> Self {
        BufferCreateContextJs(attribute_create_context)
    }
}

impl IntoJsWrapper for BufferCreateContext {
    type Result = BufferCreateContextJs;

    fn into_js_wrapper(self) -> Self::Result {
        self.into()
    }
}

impl From<&BufferCreateContext> for BufferCreateContextJs {
    fn from(attribute_create_context: &BufferCreateContext) -> Self {
        BufferCreateContextJs(attribute_create_context.to_owned())
    }
}

impl IntoJsWrapper for &BufferCreateContext {
    type Result = BufferCreateContextJs;

    fn into_js_wrapper(self) -> Self::Result {
        self.into()
    }
}
