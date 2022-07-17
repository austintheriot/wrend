use crate::{AnimationCallback, Id, IdDefault, IdName, Renderer};
use std::ops::Deref;

#[derive(Clone)]
pub struct AnimationData<
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
    id: i32,
    callback: AnimationCallback<
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
    renderer: Renderer<
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
    is_animating: bool,
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
        UserCtx: 'static,
    >
    AnimationData<
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
    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn call_animation_callback(&mut self) {
        self.callback.call(&mut self.renderer);
    }

    pub fn set_is_animating(&mut self, is_animating: bool) -> &mut Self {
        self.is_animating = is_animating;
        self
    }

    pub fn is_animating(&self) -> bool {
        self.is_animating
    }

    pub fn renderer(
        &self,
    ) -> &Renderer<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        UserCtx,
    > {
        &self.renderer
    }

    pub fn new(
        callback: AnimationCallback<
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
        renderer: Renderer<
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
            callback,
            renderer,
            id: 0,
            is_animating: false,
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
    > Deref
    for AnimationData<
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
    type Target = Renderer<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        UserCtx,
    >;

    fn deref(&self) -> &Self::Target {
        &self.renderer
    }
}
