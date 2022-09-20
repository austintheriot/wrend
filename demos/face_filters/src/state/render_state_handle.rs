use lazy_static::__Deref;

use super::render_state::RenderState;
use std::{cell::RefCell, rc::Rc, ops::{Deref, DerefMut}};

#[derive(Clone, Debug)]
pub struct RenderStateHandle(Rc<RefCell<RenderState>>);

impl From<Rc<RefCell<RenderState>>> for RenderStateHandle {
    fn from(render_state: Rc<RefCell<RenderState>>) -> Self {
        RenderStateHandle(render_state)
    }
}

impl From<RenderState> for RenderStateHandle {
    fn from(render_state: RenderState) -> Self {
        RenderStateHandle(Rc::new(RefCell::new(render_state)))
    }
}

impl Deref for RenderStateHandle {
    type Target = Rc<RefCell<RenderState>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RenderStateHandle {
    fn deref_mut (&mut self) -> &mut Rc<RefCell<RenderState>> {
        &mut self.0
    }
}