use super::{render_state::RenderState, ui_state::UiState};

#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct AppState {
    render_state: RenderState,
    ui_state: UiState,
}

impl AppState {
    pub fn render_state(&self) -> &RenderState {
        &self.render_state
    }

    pub fn render_state_mut(&mut self) -> &mut RenderState {
        &mut self.render_state
    }

    pub fn ui_state(&self) -> &UiState {
        &self.ui_state
    }

    pub fn ui_state_mut(&mut self) -> &mut UiState {
        &mut self.ui_state
    }
}