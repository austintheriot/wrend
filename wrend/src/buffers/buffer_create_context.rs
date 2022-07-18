use web_sys::WebGl2RenderingContext;

/// Context used when creating a buffer--passed as an argument to the callback
#[derive(Debug, Clone)]
pub struct BufferCreateContext<UserCtx> {
    gl: WebGl2RenderingContext,
    now: f64,
    user_ctx: Option<UserCtx>,
}

impl<UserCtx> BufferCreateContext<UserCtx> {
    /// @todo: make this into a builder pattern ?
    pub fn new(gl: WebGl2RenderingContext, now: f64, user_ctx: Option<UserCtx>) -> Self {
        Self { gl, now, user_ctx }
    }

    pub fn gl(&self) -> &WebGl2RenderingContext {
        &self.gl
    }

    pub fn now(&self) -> f64 {
        self.now
    }

    pub fn user_ctx(&self) -> &Option<UserCtx> {
        &self.user_ctx
    }
}
