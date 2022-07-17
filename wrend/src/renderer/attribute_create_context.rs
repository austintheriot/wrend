use super::attribute_location::AttributeLocation;
use web_sys::WebGl2RenderingContext;

/// This is the context object that is passed to the create_buffer callback function
#[derive(Debug, Clone)]
pub struct AttributeCreateContext<'a, UserCtx> {
    gl: &'a WebGl2RenderingContext,
    now: f64,
    attribute_location: &'a AttributeLocation,
    user_ctx: Option<&'a UserCtx>,
}

impl<'a, UserCtx> AttributeCreateContext<'a, UserCtx> {
    /// @todo: make this into a builder pattern
    pub fn new(
        gl: &'a WebGl2RenderingContext,
        now: f64,
        attribute_location: &'a AttributeLocation,
        user_ctx: Option<&'a UserCtx>,
    ) -> Self {
        Self {
            gl,
            now,
            attribute_location,
            user_ctx,
        }
    }

    pub fn gl(&self) -> &WebGl2RenderingContext {
        self.gl
    }

    pub fn now(&self) -> f64 {
        self.now
    }

    pub fn attribute_location(&self) -> &AttributeLocation {
        self.attribute_location
    }

    pub fn user_ctx(&self) -> Option<&'a UserCtx> {
        self.user_ctx
    }
}
