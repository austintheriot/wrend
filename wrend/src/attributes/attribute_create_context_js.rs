use crate::{AttributeCreateContext, AttributeLocation, IntoJsWrapper};
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

pub type AttributeCreateContextJsInner = AttributeCreateContext;

#[wasm_bindgen(js_name = AttributeCreateContext)]
pub struct AttributeCreateContextJs(AttributeCreateContextJsInner);

#[wasm_bindgen(js_class = AttributeCreateContext)]
impl AttributeCreateContextJs {
    pub fn gl(&self) -> WebGl2RenderingContext {
        self.deref().gl().to_owned()
    }

    pub fn now(&self) -> f64 {
        self.deref().now()
    }

    #[wasm_bindgen(js_name = webglBuffer)]
    pub fn webgl_buffer(&self) -> WebGlBuffer {
        self.deref().webgl_buffer().to_owned()
    }

    #[wasm_bindgen(js_name = attributeLocation)]
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
