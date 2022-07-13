use super::animation_callback::AnimationCallback;
use super::default_id::DefaultId;
use super::id::Id;
use super::id_name::IdName;
use super::renderer::Renderer;

#[derive(Clone)]
pub struct AnimationData<
    VertexShaderId: Id = DefaultId,
    FragmentShaderId: Id = DefaultId,
    ProgramId: Id = DefaultId,
    UniformId: Id + IdName = DefaultId,
    BufferId: Id + IdName = DefaultId,
    TextureId: Id = DefaultId,
    FramebufferId: Id = DefaultId,
    UserCtx: 'static = (),
> {
    id: i32,
    callback: AnimationCallback<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
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
        BufferId: Id + IdName,
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

    pub fn call_animation_callback(&self) {
        self.callback.call(&self.renderer);
    }

    pub fn set_is_animating(&mut self, is_animating: bool) -> &mut Self {
        self.is_animating = is_animating;
        self
    }

    pub fn is_animating(&self) -> bool {
        self.is_animating
    }

    pub fn new(
        callback: AnimationCallback<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
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
