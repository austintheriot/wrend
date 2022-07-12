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
    UserCtx: 'static = (),
> {
    id: i32,
    callback: AnimationCallback<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        UserCtx,
    >,
    renderer: Renderer<VertexShaderId, FragmentShaderId, ProgramId, UniformId, BufferId, UserCtx>,
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id + IdName,
        UserCtx: 'static,
    > AnimationData<VertexShaderId, FragmentShaderId, ProgramId, UniformId, BufferId, UserCtx>
{
    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn run_callback(&self) {
        self.callback.call(&self.renderer);
    }

    pub fn new(
        callback: AnimationCallback<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            UserCtx,
        >,
        renderer: Renderer<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            UserCtx,
        >,
    ) -> Self {
        Self {
            id: Default::default(),
            callback,
            renderer,
        }
    }
}
