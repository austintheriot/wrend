use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

#[derive(Debug, Clone)]
/// This is the context object that is passed to each uniform's update callback
pub struct UniformContext<UserCtx> {
    gl: WebGl2RenderingContext,
    now: f64,
    uniform_location: WebGlUniformLocation,
    user_ctx: Option<UserCtx>,
}

impl<UserCtx> UniformContext<UserCtx> {
    /// @todo: make this into a builder pattern
    pub fn new(
        gl: WebGl2RenderingContext,
        now: f64,
        uniform_location: WebGlUniformLocation,
        user_ctx: Option<UserCtx>,
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

    pub fn user_ctx(&self) -> Option<&UserCtx> {
        self.user_ctx.as_ref()
    }
}
