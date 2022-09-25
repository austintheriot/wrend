use super::{RenderCycle, UiState};

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    render_cycle: RenderCycle,
    ui_state: UiState,
    should_save: bool,
}

impl AppState {
    pub fn new(ui_state: UiState) -> Self {
        Self {
            ui_state,
            render_cycle: Default::default(),
            should_save: false,
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


    pub fn should_save(&self) -> bool {
        self.should_save
    }

    pub fn set_should_save(&mut self, should_save: bool) {
        self.should_save = should_save;
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
