use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

/// This is the context object that is passed to each uniform's update callback
pub struct UniformContext<'a, UserCtx> {
    gl: &'a WebGl2RenderingContext,
    now: f64,
    uniform_location: &'a WebGlUniformLocation,
    user_ctx: Option<&'a UserCtx>,
}

impl<'a, UserCtx> UniformContext<'a, UserCtx> {
    /// @todo: make this into a builder pattern
    pub fn new(
        gl: &'a WebGl2RenderingContext,
        now: f64,
        uniform_location: &'a WebGlUniformLocation,
        user_ctx: Option<&'a UserCtx>,
    ) -> Self {
        Self {
            gl,
            now,
            uniform_location,
            user_ctx,
        }
    }

    pub fn gl(&self) -> &WebGl2RenderingContext {
        &self.gl
    }

    pub fn now(&self) -> f64 {
        self.now
    }

    pub fn uniform_location(&self) -> &WebGlUniformLocation {
        &self.uniform_location
    }

    pub fn user_ctx(&self) -> Option<&'a UserCtx> {
        self.user_ctx
    }
}