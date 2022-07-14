use super::{
    framebuffer_create_context::FramebufferCreateContext, id::Id, id_name::IdName,
    renderer::RendererBuilder,
};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, WebGlFramebuffer};

pub type CreateFramebufferCallback<
    VertexShaderId,
    FragmentShaderId,
    ProgramId,
    UniformId,
    BufferId,
    TextureId,
    FramebufferId,
    UserCtx,
> = Rc<
    dyn Fn(
        FramebufferCreateContext<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            TextureId,
            FramebufferId,
            UserCtx,
        >,
    ) -> WebGlFramebuffer,
>;

#[derive(Clone)]
pub struct FramebufferLink<
    VertexShaderId: Id,
    FragmentShaderId: Id,
    ProgramId: Id,
    UniformId: Id + IdName,
    BufferId: Id + IdName,
    TextureId: Id,
    FramebufferId: Id,
    UserCtx: Clone + 'static,
> {
    program_id: ProgramId,
    framebuffer_id: FramebufferId,
    create_framebuffer_callback: CreateFramebufferCallback<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        TextureId,
        FramebufferId,
        UserCtx,
    >,
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        UserCtx: Clone,
    >
    FramebufferLink<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        TextureId,
        FramebufferId,
        UserCtx,
    >
where
    ProgramId: Id,
    FramebufferId: Id,
{
    pub fn new(
        program_id: ProgramId,
        framebuffer_id: FramebufferId,
        create_framebuffer_callback: CreateFramebufferCallback<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            TextureId,
            FramebufferId,
            UserCtx,
        >,
    ) -> Self {
        Self {
            program_id,
            framebuffer_id,
            create_framebuffer_callback,
        }
    }

    pub fn program_id(&self) -> &ProgramId {
        &self.program_id
    }

    pub fn framebuffer_id(&self) -> &FramebufferId {
        &self.framebuffer_id
    }

    pub fn create_framebuffer(
        &self,
        gl: &WebGl2RenderingContext,
        now: f64,
        renderer_builder: &RendererBuilder<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            TextureId,
            FramebufferId,
            UserCtx,
        >,
        user_ctx: Option<&UserCtx>,
    ) -> WebGlFramebuffer {
        let framebuffer_create_context =
            FramebufferCreateContext::new(gl, now, renderer_builder, user_ctx);
        (self.create_framebuffer_callback)(framebuffer_create_context)
    }
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        UserCtx: Clone,
    > Debug
    for FramebufferLink<
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FramebufferLink")
            .field("program_id", &self.program_id)
            .field("framebuffer_id", &self.framebuffer_id)
            .finish()
    }
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        UserCtx: Clone,
    > Hash
    for FramebufferLink<
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
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.program_id.hash(state);
        self.framebuffer_id.hash(state);
    }
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        UserCtx: Clone,
    > PartialEq
    for FramebufferLink<
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
    fn eq(&self, other: &Self) -> bool {
        self.program_id == other.program_id && self.framebuffer_id == other.framebuffer_id
    }
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        UserCtx: Clone,
    > Eq
    for FramebufferLink<
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
}
