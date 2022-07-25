use crate::state::{
    render_state::{update_cursor_position_in_world, RenderState},
    state_handle::StateHandle,
};
use web_sys::{KeyboardEvent, MouseEvent, WheelEvent};

pub fn make_handle_wheel(state_handle: StateHandle) -> Box<dyn Fn(WheelEvent)> {
    Box::new(move |e: WheelEvent| {
        let mut app_state_ref = state_handle.borrow_mut();
        let render_state = app_state_ref.render_state_mut();

        let adjustment = 1. + 0.03 * e.delta_y().signum();
        let new_value = render_state.camera_field_of_view * adjustment;
        render_state.set_fov(new_value);
    })
}

pub fn make_handle_reset(state_handle: StateHandle) -> Box<dyn Fn()> {
    Box::new(move || {
        let mut app_state_ref = state_handle.borrow_mut();
        let render_state = app_state_ref.render_state_mut();
        *render_state = RenderState::default();
    })
}

pub fn make_handle_keydown(state_handle: StateHandle) -> Box<dyn Fn(KeyboardEvent)> {
    Box::new(move |e: KeyboardEvent| {
        let mut app_state_ref = state_handle.borrow_mut();
        match e.key().as_str() {
            "w" | "W" => app_state_ref.render_state_mut().keydown_map.w = true,
            "a" | "A" => app_state_ref.render_state_mut().keydown_map.a = true,
            "s" | "S" => app_state_ref.render_state_mut().keydown_map.s = true,
            "d" | "D" => app_state_ref.render_state_mut().keydown_map.d = true,
            " " => app_state_ref.render_state_mut().keydown_map.space = true,
            "Shift" => app_state_ref.render_state_mut().keydown_map.shift = true,
            "Escape" => app_state_ref.render_state_mut().is_paused = true,
            _ => {}
        }
    })
}

pub fn make_handle_keyup(state_handle: StateHandle) -> Box<dyn Fn(KeyboardEvent)> {
    Box::new(move |e: KeyboardEvent| {
        let mut app_state_ref = state_handle.borrow_mut();
        match e.key().as_str() {
            "w" | "W" => app_state_ref.render_state_mut().keydown_map.w = false,
            "a" | "A" => app_state_ref.render_state_mut().keydown_map.a = false,
            "s" | "S" => app_state_ref.render_state_mut().keydown_map.s = false,
            "d" | "D" => app_state_ref.render_state_mut().keydown_map.d = false,
            "Shift" => app_state_ref.render_state_mut().keydown_map.shift = false,
            " " => app_state_ref.render_state_mut().keydown_map.space = false,
            _ => {}
        }
    })
}

pub fn make_handle_resize(state_handle: StateHandle) -> Box<dyn Fn()> {
    Box::new(move || {
        let mut app_state_ref = state_handle.borrow_mut();
        app_state_ref
            .render_state_mut()
            .should_update_to_match_window_size = true;
    })
}

pub fn make_handle_mouse_move(state_handle: StateHandle) -> Box<dyn Fn(MouseEvent)> {
    Box::new(move |e: MouseEvent| {
        let mut app_state_ref = state_handle.borrow_mut();
        let mut render_state_mut = app_state_ref.render_state_mut();
        // camera should move slower when more "zoomed in"
        let dx = (e.movement_x() as f64)
            * render_state_mut.look_sensitivity
            * render_state_mut.camera_field_of_view;
        let dy = -(e.movement_y() as f64)
            * render_state_mut.look_sensitivity
            * render_state_mut.camera_field_of_view;
        let yaw = render_state_mut.yaw + dx;
        let pitch = render_state_mut.pitch + dy;
        render_state_mut.set_camera_angles(yaw, pitch);
        update_cursor_position_in_world(&mut render_state_mut);
    })
}

pub fn make_handle_save(state_handle: StateHandle) -> Box<dyn Fn()> {
    Box::new(move || {
        let mut app_state_ref = state_handle.borrow_mut();
        let mut render_state_mut = app_state_ref.render_state_mut();
        render_state_mut.should_render = true;
        render_state_mut.should_save = true;
    })
}
