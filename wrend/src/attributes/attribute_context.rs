use super::attribute_location::AttributeLocation;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

/// This is the context object that is passed to each buffer's update callback
#[derive(Debug, Clone)]
pub struct AttributeContext<UserCtx: Clone> {
    gl: WebGl2RenderingContext,
    now: f64,
    buffer: WebGlBuffer,
    attribute_location: AttributeLocation,
    user_ctx: Option<UserCtx>,
}

impl<UserCtx: Clone> AttributeContext<UserCtx> {
    /// @todo: make this into a builder pattern
    pub fn new(
        gl: WebGl2RenderingContext,
        now: f64,
        buffer: WebGlBuffer,
        attribute_location: AttributeLocation,
        user_ctx: Option<UserCtx>,
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
        &self.buffer
    }

    pub fn attribute_location(&self) -> &AttributeLocation {
        &self.attribute_location
    }

    pub fn user_ctx(&self) -> Option<UserCtx> {
        self.user_ctx.clone()
    }
}
