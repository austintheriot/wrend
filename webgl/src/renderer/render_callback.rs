use super::id::Id;
use crate::renderer::renderer::Renderer;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Clone)]
pub struct RenderCallback<VertexShaderId: Id, FragmentShaderId: Id, ProgramId: Id, UserCtx> {
    callback: Rc<dyn Fn(&Renderer<VertexShaderId, FragmentShaderId, ProgramId, UserCtx>)>,
    uuid: Uuid,
}

impl<VertexShaderId: Id, FragmentShaderId: Id, ProgramId: Id, UserCtx>
    RenderCallback<VertexShaderId, FragmentShaderId, ProgramId, UserCtx>
{
    pub fn new(
        render_callback: Rc<
            dyn Fn(&Renderer<VertexShaderId, FragmentShaderId, ProgramId, UserCtx>),
        >,
    ) -> Self {
        Self {
            callback: render_callback,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn call(&self, renderer: &Renderer<VertexShaderId, FragmentShaderId, ProgramId, UserCtx>) {
        (self.callback)(renderer);
    }
}

impl<VertexShaderId: Id, FragmentShaderId: Id, ProgramId: Id, UserCtx> Hash
    for RenderCallback<VertexShaderId, FragmentShaderId, ProgramId, UserCtx>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl<VertexShaderId: Id, FragmentShaderId: Id, ProgramId: Id, UserCtx> Debug
    for RenderCallback<VertexShaderId, FragmentShaderId, ProgramId, UserCtx>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderCallback")
            .field("uuid", &self.uuid)
            .finish()
    }
}

impl<VertexShaderId: Id, FragmentShaderId: Id, ProgramId: Id, UserCtx> Default
    for RenderCallback<VertexShaderId, FragmentShaderId, ProgramId, UserCtx>
{
    fn default() -> Self {
        Self {
            callback: Rc::new(|_| {}),
            uuid: Uuid::new_v4(),
        }
    }
}

impl<VertexShaderId: Id, FragmentShaderId: Id, ProgramId: Id, UserCtx> PartialEq
    for RenderCallback<VertexShaderId, FragmentShaderId, ProgramId, UserCtx>
{
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl<VertexShaderId: Id, FragmentShaderId: Id, ProgramId: Id, UserCtx> Eq
    for RenderCallback<VertexShaderId, FragmentShaderId, ProgramId, UserCtx>
{
}
