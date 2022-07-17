use crate::renderer::renderer::Renderer;
use crate::Id;
use crate::IdDefault;
use crate::IdName;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Clone)]
pub struct RenderCallback<
    VertexShaderId: Id = IdDefault,
    FragmentShaderId: Id = IdDefault,
    ProgramId: Id = IdDefault,
    UniformId: Id + IdName = IdDefault,
    BufferId: Id = IdDefault,
    AttributeId: Id + IdName = IdDefault,
    TextureId: Id = IdDefault,
    FramebufferId: Id = IdDefault,
    UserCtx: 'static = (),
> {
    callback: Rc<
        dyn Fn(
            &Renderer<
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
        ),
    >,
    uuid: Uuid,
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
    RenderCallback<
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
        render_callback: Rc<
            dyn Fn(
                &Renderer<
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
            ),
        >,
    ) -> Self {
        Self {
            callback: render_callback,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn call(
        &self,
        renderer: &Renderer<
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
    ) {
        (self.callback)(renderer);
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
    for RenderCallback<
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
        self.uuid.hash(state);
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
    for RenderCallback<
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
        f.debug_struct("RenderCallback")
            .field("uuid", &self.uuid)
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
    > Default
    for RenderCallback<
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
    fn default() -> Self {
        Self {
            callback: Rc::new(|_| {}),
            uuid: Uuid::new_v4(),
        }
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
    for RenderCallback<
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
        self.uuid == other.uuid
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
    for RenderCallback<
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
