use std::rc::Rc;

use crate::state::app_action::AppAction;
use crate::state::app_state::AppState;
use yew::Reducible;

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next_state = (*self).clone();
        {
            let action = action;
            match action {
                _ => {}
            }
        }

        Rc::new(next_state)
    }
}
