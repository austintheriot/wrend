use crate::{IntoJsWrapper, TextureCreateContext};
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::WebGl2RenderingContext;

pub type TextureCreateContextJsInner = TextureCreateContext;

#[wasm_bindgen(js_name = TextureCreateContext)]
pub struct TextureCreateContextJs(TextureCreateContextJsInner);

#[wasm_bindgen(js_class = TextureCreateContext)]
impl TextureCreateContextJs {
    pub fn gl(&self) -> WebGl2RenderingContext {
        self.deref().gl().to_owned()
    }

    pub fn now(&self) -> f64 {
        self.deref().now()
    }
}

impl TextureCreateContextJs {
    pub fn inner(self) -> TextureCreateContextJsInner {
        self.0
    }
}

impl Deref for TextureCreateContextJs {
    type Target = TextureCreateContextJsInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TextureCreateContextJs {
    fn deref_mut(&mut self) -> &mut TextureCreateContextJsInner {
        &mut self.0
    }
}

impl From<TextureCreateContext> for TextureCreateContextJs {
    fn from(attribute_create_context: TextureCreateContext) -> Self {
        TextureCreateContextJs(attribute_create_context)
    }
}

impl IntoJsWrapper for TextureCreateContext {
    type Result = TextureCreateContextJs;

    fn into_js_wrapper(self) -> Self::Result {
        self.into()
    }
}

impl From<&TextureCreateContext> for TextureCreateContextJs {
    fn from(attribute_create_context: &TextureCreateContext) -> Self {
        TextureCreateContextJs(attribute_create_context.to_owned())
    }
}

impl IntoJsWrapper for &TextureCreateContext {
    type Result = TextureCreateContextJs;

    fn into_js_wrapper(self) -> Self::Result {
        self.into()
    }
}
