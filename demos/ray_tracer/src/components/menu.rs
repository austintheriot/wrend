use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement};
use yew::prelude::*;

use crate::components::button::Button;
use crate::state::app_context::{AppContext, AppContextError};
use crate::state::ui_state_action::UiStateAction;

#[function_component(Menu)]
pub fn menu() -> Html {
    let app_context = use_context::<AppContext>().expect(AppContextError::NOT_FOUND);
    let show_menu = app_context.ui_state.show_menu();

    if !show_menu {
        return html! {};
    }

    let handle_enable_button_click = {
        Callback::from(move |_| {
            let canvas: HtmlCanvasElement = window()
                .unwrap()
                .document()
                .unwrap()
                .query_selector("canvas")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();
            // there is a global listener that updates state in reaction to this
            canvas.request_pointer_lock();
        })
    };

    let handle_cancel_button_click = {
        let app_context = app_context.clone();
        Callback::from(move |_: MouseEvent| {
            app_context.render_state.borrow_mut().is_paused = false;
            app_context
                .ui_state
                .dispatch(UiStateAction::SetShowMenu(false));
        })
    };

    let handle_save_button_click = {
        let app_context = app_context.clone();
        Callback::from(move |_: MouseEvent| {
            app_context.render_state.borrow_mut().should_save = true;
        })
    };

    let handle_reset_button_click = {
        let app_context = app_context.clone();
        Callback::from(move |_: MouseEvent| {
            *app_context.render_state.borrow_mut() = Default::default();
        })
    };

    html! {
        <div class="menu">
            <Button onclick={handle_enable_button_click}>
                {"Enable"}
            </Button>
            <Button onclick={handle_cancel_button_click}>
                {"Cancel"}
            </Button>
            <Button onclick={handle_save_button_click}>
                {"Save Image"}
            </Button>
            <Button onclick={handle_reset_button_click}>
                {"Reset"}
            </Button>
        </div>
    }
}
