use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

#[derive(Debug, Clone)]
/// This is the context object that is passed to each uniform's update callback
pub struct UniformContext {
    gl: WebGl2RenderingContext,
    now: f64,
    uniform_location: WebGlUniformLocation,
}

impl UniformContext {
    /// @todo: make this into a builder pattern ?
    pub fn new(
        gl: WebGl2RenderingContext,
        now: f64,
        uniform_location: WebGlUniformLocation,
    ) -> Self {
        Self {
            gl,
            now,
            uniform_location,
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
}
