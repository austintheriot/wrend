use super::render_state::RenderState;
use std::{cell::RefCell, rc::Rc};

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

impl RenderStateHandle {
    pub fn get(&self) -> Rc<RefCell<RenderState>> {
        Rc::clone(&self.0)
    }
} 