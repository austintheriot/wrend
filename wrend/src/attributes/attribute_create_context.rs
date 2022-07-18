use super::attribute_location::AttributeLocation;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

/// Context that is used when a new attribute is created
#[derive(Debug, Clone)]
pub struct AttributeCreateContext<UserCtx: Clone> {
    gl: WebGl2RenderingContext,
    now: f64,
    webgl_buffer: WebGlBuffer,
    attribute_location: AttributeLocation,
    user_ctx: Option<UserCtx>,
}

impl<UserCtx: Clone> AttributeCreateContext<UserCtx> {
    /// @todo: make this into a builder pattern
    pub fn new(
        gl: WebGl2RenderingContext,
        now: f64,
        webgl_buffer: WebGlBuffer,
        attribute_location: AttributeLocation,
        user_ctx: Option<UserCtx>,
    ) -> Self {
        Self {
            gl,
            now,
            webgl_buffer,
            attribute_location,
            user_ctx,
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

    pub fn user_ctx(&self) -> Option<UserCtx> {
        self.user_ctx.clone()
    }
}
