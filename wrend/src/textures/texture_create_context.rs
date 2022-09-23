use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

/// This is the context object that is passed to the create_texture callback function
#[derive(Debug, Clone)]
pub struct TextureCreateContext {
    gl: WebGl2RenderingContext,
    now: f64,
    canvas: HtmlCanvasElement,
}

impl TextureCreateContext {
    pub fn new(gl: WebGl2RenderingContext, now: f64, canvas: HtmlCanvasElement) -> Self {
        Self { gl, now, canvas }
    }

    pub fn gl(&self) -> &WebGl2RenderingContext {
        &self.gl
    }

    pub fn now(&self) -> f64 {
        self.now
    }

    pub fn canvas(&self) -> &HtmlCanvasElement {
        &self.canvas
    }
}
