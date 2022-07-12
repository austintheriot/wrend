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
    UserCtx: 'static = (),
> {
    callback: Rc<
        dyn Fn(
            &Renderer<VertexShaderId, FragmentShaderId, ProgramId, UniformId, BufferId, UserCtx>,
        ),
    >,
}

impl<
        VertexShaderId: Id,
        FragmentShaderId: Id,
        ProgramId: Id,
        UniformId: Id + IdName,
        BufferId: Id + IdName,
        UserCtx: 'static,
    > AnimationCallback<VertexShaderId, FragmentShaderId, ProgramId, UniformId, BufferId, UserCtx>
{
    pub fn new(
        callback: Rc<
            dyn Fn(
                &Renderer<VertexShaderId, FragmentShaderId, ProgramId, UniformId, BufferId, UserCtx>,
            ),
        >,
    ) -> Self {
        Self { callback }
    }

    pub fn call(
        &self,
        renderer: &Renderer<
            VertexShaderId,
            FragmentShaderId,
            ProgramId,
            UniformId,
            BufferId,
            UserCtx,
        >,
    ) {
        (self.callback)(renderer);
    }
}
