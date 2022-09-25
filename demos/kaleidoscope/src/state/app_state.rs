use web_sys::HtmlVideoElement;
use yew::Html;

use super::{RenderCycle, UiState};

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    render_cycle: RenderCycle,
    ui_state: UiState,
    should_save: bool,
    src_video_element: HtmlVideoElement,
}

impl AppState {
    pub fn new(ui_state: UiState, src_video_element: HtmlVideoElement) -> Self {
        Self {
            ui_state,
            render_cycle: Default::default(),
            should_save: false,
            src_video_element,
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

    pub fn src_video_element(&self) -> HtmlVideoElement {
        self.src_video_element.clone()
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
