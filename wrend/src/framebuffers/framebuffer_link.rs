use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, WebGlFramebuffer};

use crate::{FramebufferCreateContext, Id, IdName, RendererBuilder};

pub type CreateFramebufferCallback<
    VertexShaderId,
    FragmentShaderId,
    ProgramId,
    UniformId,
    BufferId,
    AttributeId,
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
            AttributeId,
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
    BufferId: Id,
    AttributeId: Id + IdName,
    TextureId: Id,
    FramebufferId: Id,
    UserCtx: 'static,
> {
    framebuffer_id: FramebufferId,
    create_framebuffer_callback: CreateFramebufferCallback<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
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
        BufferId: Id,
        AttributeId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        UserCtx,
    >
    FramebufferLink<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        UserCtx,
    >
{
    pub fn new(
        framebuffer_id: FramebufferId,
        create_framebuffer_callback: CreateFramebufferCallback<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            AttributeId,
            TextureId,
            FramebufferId,
            UserCtx,
        >,
    ) -> Self {
        Self {
            framebuffer_id,
            create_framebuffer_callback,
        }
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
            AttributeId,
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
        BufferId: Id,
        AttributeId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        UserCtx,
    > Debug
    for FramebufferLink<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        UserCtx,
    >
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FramebufferLink")
            .field("framebuffer_id", &self.framebuffer_id)
            .finish()
    }
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id,
        AttributeId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        UserCtx,
    > Hash
    for FramebufferLink<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        UserCtx,
    >
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.framebuffer_id.hash(state);
    }
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id,
        AttributeId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        UserCtx,
    > PartialEq
    for FramebufferLink<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        UserCtx,
    >
{
    fn eq(&self, other: &Self) -> bool {
        self.framebuffer_id == other.framebuffer_id
    }
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id,
        AttributeId: Id + IdName,
        TextureId: Id,
        FramebufferId: Id,
        UserCtx,
    > Eq
    for FramebufferLink<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        UserCtx,
    >
{
}
