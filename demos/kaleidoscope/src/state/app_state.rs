use super::{RenderCycle, UiState};

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    render_cycle: RenderCycle,
    ui_state: UiState,
}

impl AppState {
    pub fn new(ui_state: UiState) -> Self {
        Self {
            ui_state,
            render_cycle: Default::default(),
        }
    }

    pub fn current_render_cycle(&self) -> RenderCycle {
        self.render_cycle
    }

    pub fn advance_render_cycle(&mut self) {
        self.render_cycle = self.render_cycle.next();
    }

    pub fn ui_state(&self) -> &UiState {
        self.as_ref()
    }
}

impl AsRef<UiState> for AppState {
    fn as_ref(&self) -> &UiState {
        &self.ui_state
    }
}

impl AsMut<UiState> for AppState {
    fn as_mut(&mut self) -> &mut UiState {
        &mut self.ui_state
    }
}