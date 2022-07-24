use super::{render_state::RenderState, ui_state::UiState};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct AppState {
    render_state: RenderState,
    ui_state: UiState,
}