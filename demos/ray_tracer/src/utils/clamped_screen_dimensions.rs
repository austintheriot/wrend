use web_sys::window;

/// This constant can be updated to limit the screen dimensions to something smaller
pub const MAX_CANVAS_SIZE: u32 = u32::MAX;

// Returns the width of the screen or `MAX_CANVAS_SIZE`--whichever is smaller.
// Limits the canvas dimensions to a reasonable number
// (to prevent off-the-charts GPU work on large screen sizes)
pub fn clamped_screen_dimensions() -> (u32, u32) {
    let window = window().unwrap();
    let raw_screen_width = window.inner_width().unwrap().as_f64().unwrap();
    let raw_screen_height = window.inner_height().unwrap().as_f64().unwrap();
    let aspect_ratio = raw_screen_width / raw_screen_height;

    if raw_screen_width > raw_screen_height {
        let adjusted_width = raw_screen_width.min(MAX_CANVAS_SIZE as f64);
        let adjusted_height = adjusted_width / aspect_ratio;
        (adjusted_width as u32, adjusted_height as u32)
    } else {
        let adjusted_height = raw_screen_width.min(MAX_CANVAS_SIZE as f64);
        let adjusted_width = adjusted_height * aspect_ratio;
        (adjusted_width as u32, adjusted_height as u32)
    }
}
