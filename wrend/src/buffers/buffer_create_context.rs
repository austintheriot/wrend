use web_sys::WebGl2RenderingContext;

/// This is the context object that is passed to the create_texture callback function
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
