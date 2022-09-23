use super::app_state::AppState;
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

#[derive(Clone, Debug)]
pub struct AppStateHandle(Rc<RefCell<AppState>>);

impl From<Rc<RefCell<AppState>>> for AppStateHandle {
    fn from(app_state: Rc<RefCell<AppState>>) -> Self {
        AppStateHandle(app_state)
    }
}

impl From<AppState> for AppStateHandle {
    fn from(app_state: AppState) -> Self {
        AppStateHandle(Rc::new(RefCell::new(app_state)))
    }
}

impl Deref for AppStateHandle {
    type Target = Rc<RefCell<AppState>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AppStateHandle {
    fn deref_mut(&mut self) -> &mut Rc<RefCell<AppState>> {
        &mut self.0
    }
}
