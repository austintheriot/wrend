use super::app_state::AppState;
use std::{cell::RefCell, rc::Rc};
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct StateHandle(Rc<RefCell<AppState>>);

impl StateHandle {
    pub fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        Self(app_state)
    }
}

impl From<Rc<RefCell<AppState>>> for StateHandle {
    fn from(app_state: Rc<RefCell<AppState>>) -> Self {
        StateHandle(app_state)
    }
}

impl From<AppState> for StateHandle {
    fn from(app_state: AppState) -> Self {
        StateHandle(Rc::new(RefCell::new(app_state)))
    }
}

impl Deref for StateHandle {
    type Target = Rc<RefCell<AppState>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
