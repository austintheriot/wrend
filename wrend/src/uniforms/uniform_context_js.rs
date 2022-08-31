use crate::{UniformContext, AttributeLocation, IntoJsWrapper};
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlUniformLocation};

pub type UniformContextJsInner = UniformContext;

#[wasm_bindgen(js_name = UniformContext)]
pub struct UniformContextJs(UniformContextJsInner);

#[wasm_bindgen(js_class = UniformContext)]
impl UniformContextJs {
    pub fn gl(&self) -> WebGl2RenderingContext {
        self.deref().gl().to_owned()
    }

    pub fn now(self) -> f64 {
        self.deref().now()
    }

    #[wasm_bindgen(js_name = uniformLocation)]
    pub fn uniform_location(self) -> WebGlUniformLocation {
        self.deref().uniform_location().to_owned()
    }
}

impl UniformContextJs {
    pub fn inner(self) -> UniformContextJsInner {
        self.0
    }
}

impl Deref for UniformContextJs {
    type Target = UniformContextJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UniformContextJs {
    fn deref_mut(&mut self) -> &mut UniformContextJsInner {
        &mut self.0
    }
}

impl From<UniformContext> for UniformContextJs {
    fn from(attribute_create_context: UniformContext) -> Self {
        UniformContextJs(attribute_create_context)
    }
}

impl IntoJsWrapper for UniformContext {
    type Result = UniformContextJs;

    fn into_js_wrapper(self) -> Self::Result {
        self.into()
    }
}

impl From<&UniformContext> for UniformContextJs {
    fn from(attribute_create_context: &UniformContext) -> Self {
        UniformContextJs(attribute_create_context.to_owned())
    }
}

impl IntoJsWrapper for &UniformContext {
    type Result = UniformContextJs;

    fn into_js_wrapper(self) -> Self::Result {
        self.into()
    }
}
