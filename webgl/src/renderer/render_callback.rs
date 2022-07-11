use crate::renderer::renderer::Renderer;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Clone)]
pub struct RenderCallback<Id, UserCtx>
where
    Id: Hash + Eq + Clone + Debug + Default,
{
    callback: Rc<dyn Fn(&Renderer<Id, UserCtx>)>,
    uuid: Uuid,
}

impl<Id, UserCtx> RenderCallback<Id, UserCtx>
where
    Id: Hash + Eq + Clone + Debug + Default,
{
    pub fn new(render_callback: Rc<dyn Fn(&Renderer<Id, UserCtx>)>) -> Self {
        Self {
            callback: render_callback,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn call(&self, renderer: &Renderer<Id, UserCtx>) {
        (self.callback)(renderer);
    }
}

impl<Id, UserCtx> Hash for RenderCallback<Id, UserCtx>
where
    Id: Hash + Eq + Clone + Debug + Default,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl<Id, UserCtx> Debug for RenderCallback<Id, UserCtx>
where
    Id: Hash + Eq + Clone + Debug + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderCallback")
            .field("uuid", &self.uuid)
            .finish()
    }
}

impl<Id, UserCtx> Default for RenderCallback<Id, UserCtx>
where
    Id: Hash + Eq + Clone + Debug + Default,
{
    fn default() -> Self {
        Self {
            callback: Rc::new(|_| {}),
            uuid: Uuid::new_v4(),
        }
    }
}

impl<Id, UserCtx> PartialEq for RenderCallback<Id, UserCtx>
where
    Id: Hash + Eq + Clone + Debug + Default,
{
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl<Id, UserCtx> Eq for RenderCallback<Id, UserCtx> where Id: Hash + Eq + Clone + Debug + Default {}
