use crate::state::{app_context::AppContext, ui_state_action::UiStateAction};
use wasm_bindgen::JsValue;
use web_sys::{Document, Event, HtmlCanvasElement, KeyboardEvent, MouseEvent, WheelEvent};

pub fn make_handle_wheel(app_context: AppContext) -> impl Fn(WheelEvent) + 'static {
    move |e: WheelEvent| {
        if app_context.render_state.borrow().is_paused() {
            return;
        }

        let mut render_state = app_context.render_state.borrow_mut();
        let adjustment = 1. + 0.03 * e.delta_y().signum();
        let new_camera_field_of_view = render_state.pipeline().camera_field_of_view() * adjustment;
        render_state
            .pipeline_mut()
            .set_camera_field_of_view(new_camera_field_of_view);
    }
}

pub fn make_handle_keydown(app_context: AppContext) -> impl Fn(KeyboardEvent) + 'static {
    move |e: KeyboardEvent| {
        if app_context.render_state.borrow().is_paused() {
            return;
        }

        if let Ok(keydown_key) = e.key().try_into() {
            let mut render_state = app_context.render_state.borrow_mut();
            render_state.keydown_state_mut()[keydown_key] = true;
        }

        if let "Escape" = e.key().as_str() {
            app_context.render_state.borrow_mut().set_is_paused(true);
            app_context
                .ui_state
                .dispatch(UiStateAction::SetShowMenu(true));
        }
    }
}

pub fn make_handle_keyup(app_context: AppContext) -> impl Fn(KeyboardEvent) + 'static {
    move |e: KeyboardEvent| {
        if let Ok(keydown_key) = e.key().try_into() {
            let mut render_state = app_context.render_state.borrow_mut();
            render_state.keydown_state_mut()[keydown_key] = false;
        }
    }
}

pub fn make_handle_resize(app_context: AppContext) -> impl Fn(JsValue) + 'static {
    move |_| {
        app_context
            .render_state
            .borrow_mut()
            .set_should_update_to_match_window_size(true);
    }
}

pub fn make_handle_mouse_move(app_context: AppContext) -> impl Fn(MouseEvent) + 'static {
    move |e: MouseEvent| {
        if app_context.render_state.borrow().is_paused() {
            return;
        }

        let mut render_state = app_context.render_state.borrow_mut();
        let look_sensitivity = render_state.look_sensitivity();
        let camera_field_of_view = render_state.pipeline().camera_field_of_view();

        // camera should move slower when more "zoomed in"
        let dx = (e.movement_x() as f64) * look_sensitivity * camera_field_of_view;
        let dy = -(e.movement_y() as f64) * look_sensitivity * camera_field_of_view;
        let yaw = render_state.pipeline().yaw() + dx;
        let pitch = render_state.pipeline().pitch() + dy;

        render_state.pipeline_mut().set_pitch_and_yaw(pitch, yaw);
        render_state.update_cursor_position_in_world();
    }
}
pub fn make_handle_pointer_lock_change(
    app_context: AppContext,
    document: Document,
    canvas: HtmlCanvasElement,
) -> impl Fn(Event) + 'static {
    move |_: Event| {
        if let Some(pointer_lock_element) = document.pointer_lock_element() {
            if &pointer_lock_element == canvas.as_ref() {
                app_context.render_state.borrow_mut().set_is_paused(false);
                app_context
                    .ui_state
                    .dispatch(UiStateAction::SetShowMenu(false));
                return;
            }
        }
        app_context.render_state.borrow_mut().set_is_paused(true);
        app_context
            .ui_state
            .dispatch(UiStateAction::SetShowMenu(true));
    }
}
