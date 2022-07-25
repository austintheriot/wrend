use super::{render_state::RenderState, ui_state::UiState};
use std::{cell::RefCell, rc::Rc};
use yew::{use_mut_ref, use_reducer_eq, UseReducerHandle};

pub type UiStateHandle = UseReducerHandle<UiState>;
pub type RenderStateHandle = Rc<RefCell<RenderState>>;

#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    pub ui_state: UiStateHandle,
    pub render_state: RenderStateHandle,
}

impl Default for AppContext {
    fn default() -> Self {
        AppContext {
            ui_state: use_reducer_eq(UiState::default),
            render_state: use_mut_ref(RenderState::default),
        }
    }
}

pub struct AppContextError;

impl AppContextError {
    pub const NOT_FOUND: &'static str = "AppContext was not found";
}
