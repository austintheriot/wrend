use std::rc::Rc;

use super::{
    attribute_id::AttributeId, buffer_id::BufferId, fragment_shader_id::FragmentShaderId,
    framebuffer_id::FramebufferId, program_id::ProgramId, texture_id::TextureId,
    transform_feedback_id::TransformFeedbackId, vao_id::VAOId, vertex_shader_id::VertexShaderId,
};
use crate::state::{app_context::AppContext, render_state::RESIZE_UPDATE_DEBOUNCE_INTERVAL};
use web_sys::{window, WebGlTexture};
use wrend::Renderer;

/// This callback is called on every frame of the global animation cycle
pub fn animate(
    renderer: &Renderer<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        String,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        TransformFeedbackId,
        VAOId,
        AppContext,
    >,
) {
    let gl = renderer.gl();
    let canvas = renderer.canvas();
    let render_state = Rc::clone(&renderer.user_ctx().as_ref().unwrap().render_state);
    let all_textures: Vec<WebGlTexture> = renderer
        .textures()
        .values()
        .into_iter()
        .map(|texture| texture.webgl_texture().clone())
        .collect();
    let now = window().unwrap().performance().unwrap().now();
    let dt = now - render_state.borrow().prev_now();
    {
        let mut render_state = render_state.borrow_mut();
        render_state.set_prev_now(now);
        render_state.update_position(dt);
    }

    // don't render while paused unless trying to save
    // OR unless it's the very first frame
    let should_render = {
        let render_state = render_state.borrow();
        !render_state.is_paused()
            || render_state.should_save_image()
            || render_state.render_count() == 0
    };

    let should_run_resize_fn = {
        let render_state = render_state.borrow();
        (render_state.window_size_out_of_sync()
            && now - render_state.prev_resize_sync_time() > RESIZE_UPDATE_DEBOUNCE_INTERVAL)
            || render_state.render_count() == 0
    };

    // debounce resize handler
    if should_run_resize_fn {
        let mut render_state = render_state.borrow_mut();
        render_state.set_window_size_out_of_sync(false);
        render_state.sync_dimensions(gl, &all_textures, canvas, now);
    }

    if should_render {
        render_state.borrow_mut().inc_count();
        render_state.borrow_mut().inc_render_count();
        renderer.update_uniforms();
        renderer.render();

        {
            let mut render_state = render_state.borrow_mut();
            if render_state.should_save_image() {
                render_state.set_should_save_image(false);
                renderer.save_image();
            }
        }
    }
}