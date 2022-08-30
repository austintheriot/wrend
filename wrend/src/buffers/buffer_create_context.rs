use web_sys::WebGl2RenderingContext;

/// Context used when creating a buffer--passed as an argument to the callback
#[derive(Debug, Clone)]
pub struct BufferCreateContext {
    gl: WebGl2RenderingContext,
    now: f64,
}

impl BufferCreateContext {
    pub fn new(gl: WebGl2RenderingContext, now: f64) -> Self {
        Self { gl, now }
    }

    pub fn gl(&self) -> &WebGl2RenderingContext {
        &self.gl
    }

    pub fn now(&self) -> f64 {
        self.now
    }
}
