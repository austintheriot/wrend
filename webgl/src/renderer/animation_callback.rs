use super::default_id::DefaultId;
use super::id::Id;
use super::id_name::IdName;
use super::renderer::Renderer;
use std::rc::Rc;

#[derive(Clone)]
pub struct AnimationCallback<
    VertexShaderId: Id = DefaultId,
    FragmentShaderId: Id = DefaultId,
    ProgramId: Id = DefaultId,
    UniformId: Id + IdName = DefaultId,
    BufferId: Id + IdName = DefaultId,
    TextureId: Id = DefaultId,
    FramebufferId: Id = DefaultId,
    UserCtx: Clone + 'static = (),
> {
    callback: Rc<
        dyn Fn(
            &mut Renderer<
                VertexShaderId,
                FragmentShaderId,
                ProgramId,
                UniformId,
                BufferId,
                TextureId,
                FramebufferId,
                UserCtx,
            >,
        ),
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
        UserCtx: Clone + 'static,
    >
    AnimationCallback<
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
    pub fn new(
        callback: Rc<
            dyn Fn(
                &mut Renderer<
                    VertexShaderId,
                    FragmentShaderId,
                    ProgramId,
                    UniformId,
                    BufferId,
                    TextureId,
                    FramebufferId,
                    UserCtx,
                >,
            ),
        >,
    ) -> Self {
        Self { callback }
    }

    pub fn call(
        &self,
        renderer: &mut Renderer<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            TextureId,
            FramebufferId,
            UserCtx,
        >,
    ) {
        (self.callback)(renderer);
    }
}
