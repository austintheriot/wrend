use crate::{AttributeCreateContext, AttributeLocation, IntoJsWrapper};
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

pub type AttributeCreateContextJsInner = AttributeCreateContext;

#[wasm_bindgen(js_name = AttributeCreateContext)]
pub struct AttributeCreateContextJs(AttributeCreateContextJsInner);

#[wasm_bindgen(js_class = AttributeCreateContext)]
impl AttributeCreateContextJs {
    #[wasm_bindgen(constructor)]
    pub fn new(
        gl: WebGl2RenderingContext,
        now: f64,
        webgl_buffer: WebGlBuffer,
        attribute_location: AttributeLocation,
    ) -> Self {
        Self(AttributeCreateContextJsInner::new(
            gl,
            now,
            webgl_buffer,
            attribute_location,
        ))
    }

    pub fn gl(&self) -> WebGl2RenderingContext {
        self.deref().gl().to_owned()
    }

    pub fn now(&self) -> f64 {
        self.deref().now()
    }

    pub fn webgl_buffer(&self) -> WebGlBuffer {
        self.deref().webgl_buffer().to_owned()
    }

    pub fn attribute_location(&self) -> AttributeLocation {
        self.deref().attribute_location().to_owned()
    }
}

impl AttributeCreateContextJs {
    pub fn inner(self) -> AttributeCreateContextJsInner {
        self.0
    }
}

impl Deref for AttributeCreateContextJs {
    type Target = AttributeCreateContextJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AttributeCreateContextJs {
    fn deref_mut(&mut self) -> &mut AttributeCreateContextJsInner {
        &mut self.0
    }
}

impl From<AttributeCreateContext> for AttributeCreateContextJs {
    fn from(attribute_create_context: AttributeCreateContext) -> Self {
        AttributeCreateContextJs(attribute_create_context)
    }
}

impl IntoJsWrapper for AttributeCreateContext {
    type Result = AttributeCreateContextJs;

    fn into_js_wrapper(self) -> Self::Result {
        self.into()
    }
}

impl From<&AttributeCreateContext> for AttributeCreateContextJs {
    fn from(attribute_create_context: &AttributeCreateContext) -> Self {
        AttributeCreateContextJs(attribute_create_context.to_owned())
    }
}

impl IntoJsWrapper for &AttributeCreateContext {
    type Result = AttributeCreateContextJs;

    fn into_js_wrapper(self) -> Self::Result {
        self.into()
    }
}
