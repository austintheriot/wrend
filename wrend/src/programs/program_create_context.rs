use web_sys::{WebGl2RenderingContext, WebGlShader};

#[derive(Debug, Clone)]
pub struct ProgramCreateContext<UserCtx> {
    gl: WebGl2RenderingContext,
    now: f64,
    user_ctx: Option<UserCtx>,
    fragment_shader: WebGlShader,
    vertex_shader: WebGlShader,
    transform_feedback_varyings: Vec<String>,
}

impl<UserCtx> ProgramCreateContext<UserCtx> {
    /// @todo: make this into a builder pattern
    pub fn new(
        gl: WebGl2RenderingContext,
        now: f64,
        user_ctx: Option<UserCtx>,
        fragment_shader: WebGlShader,
        vertex_shader: WebGlShader,
        transform_feedback_varyings: Vec<String>,
    ) -> Self {
        Self {
            gl,
            now,
            user_ctx,
            fragment_shader,
            vertex_shader,
            transform_feedback_varyings,
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

    pub fn vertex_shader(&self) -> &WebGlShader {
        &self.vertex_shader
    }

    pub fn fragment_shader(&self) -> &WebGlShader {
        &self.fragment_shader
    }

    pub fn transform_feedback_varyings(&self) -> &Vec<String> {
        &self.transform_feedback_varyings
    }
}
