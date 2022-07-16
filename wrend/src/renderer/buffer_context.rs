use web_sys::{WebGl2RenderingContext, WebGlBuffer};
use super::attribute_location::AttributeLocation;

/// This is the context object that is passed to each buffer's update callback
#[derive(Debug, Clone)]
pub struct BufferContext<'a, UserCtx> {
    gl: &'a WebGl2RenderingContext,
    now: f64,
    buffer: &'a WebGlBuffer,
    attribute_location: AttributeLocation,
    user_ctx: Option<&'a UserCtx>,
}

impl<'a, UserCtx> BufferContext<'a, UserCtx> {
    /// @todo: make this into a builder pattern
    pub fn new(
        gl: &'a WebGl2RenderingContext,
        now: f64,
        buffer: &'a WebGlBuffer,
        attribute_location: AttributeLocation,
        user_ctx: Option<&'a UserCtx>,
    ) -> Self {
        Self {
            gl,
            now,
            buffer,
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

    pub fn buffer(&self) -> &WebGlBuffer {
        self.buffer
    }

    pub fn attribute_location(&self) -> &AttributeLocation {
        &self.attribute_location
    }

    pub fn user_ctx(&self) -> Option<&'a UserCtx> {
        self.user_ctx
    }
}