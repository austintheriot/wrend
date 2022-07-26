use std::rc::Rc;

use crate::{
    controls::Listener,
    state::{
        app_context::{AppContext, AppContextError},
        ui_state_action::UiStateAction,
    },
};
use web_sys::window;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct KeyboardListenerProps {
    pub children: Children,
}

#[function_component(KeyboardListener)]
pub fn keyboard_listener(props: &KeyboardListenerProps) -> Html {
    let app_context = use_context::<AppContext>().expect(AppContextError::NOT_FOUND);
    let listener_mut_ref = use_mut_ref(|| None);
    use_effect_with_deps(
        move |_| {
            let keydown_listener = {
                let listener_mut_ref = Rc::clone(&listener_mut_ref);
                Listener::new(
                    window().unwrap().as_ref(),
                    "keydown",
                    move |e: KeyboardEvent| {
                        if app_context.ui_state.is_keyboard_user() {
                            return;
                        }

                        // once we know the use is keyboard listener, no reason to keep listening
                        listener_mut_ref.borrow_mut().take();
                        if let "Tab" = e.key().as_str() {
                            app_context
                                .ui_state
                                .dispatch(UiStateAction::SetIsKeyboardUser);
                        }
                    },
                )
            };

            // keep callback valid for component lifecycle
            // when it the component is unmounted, the listener will be dropped,
            // and the listener will be removed from the window
            listener_mut_ref.borrow_mut().replace(keydown_listener);

            || {}
        },
        (),
    );

    html! {
     <>
         {for props.children.iter()}
     </>
    }
}
