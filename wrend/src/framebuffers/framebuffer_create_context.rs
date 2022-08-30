use web_sys::{WebGl2RenderingContext, WebGlTexture};

/// Context used when creating a Framebuffer object--passed into the callback as the first argument
#[derive(Debug, Clone)]
pub struct FramebufferCreateContext {
    gl: WebGl2RenderingContext,
    now: f64,
    /// This is the texture that was specified in the link
    /// and which will be associated with the Framebuffer
    webgl_texture: Option<WebGlTexture>,
}

impl FramebufferCreateContext {
    /// @todo: make this into a builder pattern
    pub fn new(gl: WebGl2RenderingContext, now: f64, webgl_texture: Option<WebGlTexture>) -> Self {
        Self {
            gl,
            now,
            webgl_texture,
        }
    }

    pub fn gl(&self) -> &WebGl2RenderingContext {
        &self.gl
    }

    pub fn now(&self) -> f64 {
        self.now
    }

    pub fn webgl_texture(&self) -> &Option<WebGlTexture> {
        &self.webgl_texture
    }
}
