use super::{ui_state::UiState, ui_state_action::UiStateAction};
use std::rc::Rc;
use yew::Reducible;

impl Reducible for UiState {
    type Action = UiStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next_state = *self;
        match action {
            UiStateAction::SetIsKeyboardUser => {
                next_state.set_is_keyboard_user();
            }
            UiStateAction::SetShowMenu(show_menu) => {
                next_state.set_show_menu(show_menu);
            },
        }
        Rc::new(next_state)
    }
}
