use web_sys::WebGl2RenderingContext;

use super::{id::Id, id_name::IdName, renderer::Renderer};

/// This is the context object that is passed to the create_texture callback function
#[derive(Debug, Clone)]
pub struct FramebufferCreateContext<
    'a,
    VertexShaderId: Id,
    FragmentShaderId: Id,
    ProgramId: Id,
    UniformId: Id + IdName,
    BufferId: Id + IdName,
    TextureId: Id,
    FramebufferId: Id,
    UserCtx: 'static,
> {
    gl: &'a WebGl2RenderingContext,
    now: f64,
    renderer: &'a Renderer<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        TextureId,
        FramebufferId,
        UserCtx,
    >,
    user_ctx: Option<&'a UserCtx>,
}

impl<
        'a,
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        UserCtx,
    >
    FramebufferCreateContext<
        'a,
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        TextureId,
        FramebufferId,
        UserCtx,
    >
{
    /// @todo: make this into a builder pattern
    pub fn new(
        gl: &'a WebGl2RenderingContext,
        now: f64,
        renderer: &'a Renderer<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            TextureId,
            FramebufferId,
            UserCtx,
        >,
        user_ctx: Option<&'a UserCtx>,
    ) -> Self {
        Self {
            gl,
            now,
            user_ctx,
            renderer,
        }
    }

    pub fn gl(&self) -> &WebGl2RenderingContext {
        self.gl
    }

    pub fn now(&self) -> f64 {
        self.now
    }

    pub fn user_ctx(&self) -> Option<&'a UserCtx> {
        self.user_ctx
    }

    pub fn renderer(
        &self,
    ) -> &'a Renderer<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        TextureId,
        FramebufferId,
        UserCtx,
    > {
        self.renderer
    }
}
