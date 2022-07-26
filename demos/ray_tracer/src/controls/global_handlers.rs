use crate::state::{
    app_context::AppContext, render_state::update_cursor_position_in_world,
    ui_state_action::UiStateAction,
};
use wasm_bindgen::JsValue;
use web_sys::{Document, Event, HtmlCanvasElement, KeyboardEvent, MouseEvent, WheelEvent};

pub fn make_handle_wheel(app_context: AppContext) -> impl Fn(WheelEvent) + 'static {
    move |e: WheelEvent| {
        if app_context.render_state.borrow().is_paused {
            return;
        }

        let mut render_state = app_context.render_state.borrow_mut();
        let adjustment = 1. + 0.03 * e.delta_y().signum();
        let new_value = render_state.camera_field_of_view * adjustment;
        render_state.set_fov(new_value);
    }
}

pub fn make_handle_keydown(app_context: AppContext) -> impl Fn(KeyboardEvent) + 'static {
    move |e: KeyboardEvent| {
        if app_context.render_state.borrow().is_paused {
            return;
        }

        match e.key().as_str() {
            "w" | "W" => app_context.render_state.borrow_mut().keydown_map.w = true,
            "a" | "A" => app_context.render_state.borrow_mut().keydown_map.a = true,
            "s" | "S" => app_context.render_state.borrow_mut().keydown_map.s = true,
            "d" | "D" => app_context.render_state.borrow_mut().keydown_map.d = true,
            " " => app_context.render_state.borrow_mut().keydown_map.space = true,
            "Shift" => app_context.render_state.borrow_mut().keydown_map.shift = true,
            "Escape" => {
                app_context.render_state.borrow_mut().is_paused = true;
                app_context
                    .ui_state
                    .dispatch(UiStateAction::SetShowMenu(true));
            }
            _ => {}
        }
    }
}

pub fn make_handle_keyup(app_context: AppContext) -> impl Fn(KeyboardEvent) + 'static {
    move |e: KeyboardEvent| {
        let mut render_state = app_context.render_state.borrow_mut();
        match e.key().as_str() {
            "w" | "W" => render_state.keydown_map.w = false,
            "a" | "A" => render_state.keydown_map.a = false,
            "s" | "S" => render_state.keydown_map.s = false,
            "d" | "D" => render_state.keydown_map.d = false,
            "Shift" => render_state.keydown_map.shift = false,
            " " => render_state.keydown_map.space = false,
            _ => {}
        }
    }
}

pub fn make_handle_resize(app_context: AppContext) -> impl Fn(JsValue) + 'static {
    move |_| {
        let mut render_state = app_context.render_state.borrow_mut();
        render_state.should_update_to_match_window_size = true;
    }
}

pub fn make_handle_mouse_move(app_context: AppContext) -> impl Fn(MouseEvent) + 'static {
    move |e: MouseEvent| {
        if app_context.render_state.borrow().is_paused {
            return;
        }

        let mut render_state = app_context.render_state.borrow_mut();
        // camera should move slower when more "zoomed in"
        let dx = (e.movement_x() as f64)
            * render_state.look_sensitivity
            * render_state.camera_field_of_view;
        let dy = -(e.movement_y() as f64)
            * render_state.look_sensitivity
            * render_state.camera_field_of_view;
        let yaw = render_state.yaw + dx;
        let pitch = render_state.pitch + dy;
        render_state.set_camera_angles(yaw, pitch);
        update_cursor_position_in_world(&mut render_state);
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
                app_context.render_state.borrow_mut().is_paused = false;
                app_context
                    .ui_state
                    .dispatch(UiStateAction::SetShowMenu(false));
                return;
            }
        }
        app_context.render_state.borrow_mut().is_paused = true;
        app_context
            .ui_state
            .dispatch(UiStateAction::SetShowMenu(true));
    }
}
