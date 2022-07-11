use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

pub struct UniformContext<'a, UserCtx> {
    gl: &'a WebGl2RenderingContext,
    now: f64,
    uniform_location: &'a WebGlUniformLocation,
    user_ctx: Option<&'a UserCtx>,
}

impl<'a, UserCtx> UniformContext<'a, UserCtx> {
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
