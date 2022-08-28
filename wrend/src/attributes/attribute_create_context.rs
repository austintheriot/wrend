use super::attribute_location::AttributeLocation;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

/// Context that is used when a new attribute is created
#[derive(Debug, Clone)]
pub struct AttributeCreateContext {
    gl: WebGl2RenderingContext,
    now: f64,
    webgl_buffer: WebGlBuffer,
    attribute_location: AttributeLocation,
}

impl AttributeCreateContext {
    /// @todo: make this into a builder pattern
    pub fn new(
        gl: WebGl2RenderingContext,
        now: f64,
        webgl_buffer: WebGlBuffer,
        attribute_location: AttributeLocation,
    ) -> Self {
        Self {
            gl,
            now,
            webgl_buffer,
            attribute_location,
        }
    }

    pub fn gl(&self) -> &WebGl2RenderingContext {
        &self.gl
    }

    pub fn now(&self) -> f64 {
        self.now
    }

    pub fn webgl_buffer(&self) -> &WebGlBuffer {
        &self.webgl_buffer
    }

    pub fn attribute_location(&self) -> &AttributeLocation {
        &self.attribute_location
    }
}
