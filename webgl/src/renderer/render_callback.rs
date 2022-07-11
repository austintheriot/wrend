use crate::renderer::renderer::Renderer;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Clone)]
pub struct RenderCallback<I>
where
    I: Hash + Eq + Clone + Debug + Default,
{
    callback: Rc<dyn Fn(&Renderer<I>)>,
    uuid: Uuid,
}

impl<I> RenderCallback<I>
where
    I: Hash + Eq + Clone + Debug + Default,
{
    pub fn new(render_callback: Rc<dyn Fn(&Renderer<I>)>) -> Self {
        Self {
            callback: render_callback,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn call(&self, renderer: &Renderer<I>) {
        (self.callback)(renderer);
    }
}

impl<I> Hash for RenderCallback<I>
where
    I: Hash + Eq + Clone + Debug + Default,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl<I> Debug for RenderCallback<I>
where
    I: Hash + Eq + Clone + Debug + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderCallback")
            .field("uuid", &self.uuid)
            .finish()
    }
}

impl<I> Default for RenderCallback<I>
where
    I: Hash + Eq + Clone + Debug + Default,
{
    fn default() -> Self {
        Self {
            callback: Rc::new(|_| {}),
            uuid: Uuid::new_v4(),
        }
    }
}

impl<I> PartialEq for RenderCallback<I>
where
    I: Hash + Eq + Clone + Debug + Default,
{
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl<I> Eq for RenderCallback<I> where I: Hash + Eq + Clone + Debug + Default {}
