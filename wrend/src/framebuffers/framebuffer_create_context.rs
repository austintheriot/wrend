use web_sys::{WebGl2RenderingContext, WebGlTexture};

/// This is the context object that is passed to the create_texture callback function
#[derive(Debug, Clone)]
pub struct FramebufferCreateContext<UserCtx: Clone + 'static> {
    gl: WebGl2RenderingContext,
    now: f64,
    /// This is the texture that was specified in the link
    /// and which will be associated with the Framebuffer
    webgl_texture: Option<WebGlTexture>,
    user_ctx: Option<UserCtx>,
}

impl<UserCtx: Clone> FramebufferCreateContext<UserCtx> {
    /// @todo: make this into a builder pattern
    pub fn new(
        gl: WebGl2RenderingContext,
        now: f64,
        texture: Option<WebGlTexture>,
        user_ctx: Option<UserCtx>,
    ) -> Self {
        Self {
            gl,
            now,
            webgl_texture: texture,
            user_ctx,
        }
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

    pub fn webgl_texture(&self) -> &Option<WebGlTexture> {
        &self.webgl_texture
    }
}
