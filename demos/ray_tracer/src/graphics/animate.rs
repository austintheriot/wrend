use std::rc::Rc;

use super::{
    attribute_id::AttributeId, buffer_id::BufferId, fragment_shader_id::FragmentShaderId,
    framebuffer_id::FramebufferId, program_id::ProgramId, texture_id::TextureId,
    transform_feedback_id::TransformFeedbackId, vao_id::VAOId, vertex_shader_id::VertexShaderId,
};
use crate::state::{app_context::AppContext, render_state::RESIZE_UPDATE_DEBOUNCE_INTERVAL};
use web_sys::{window, WebGlTexture};
use wrend::RendererData;

/// This callback is called on every frame of the global animation cycle
pub fn animate(
    renderer_data: &RendererData<
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
    let gl = renderer_data.gl();
    let canvas = renderer_data.canvas();
    let render_state = Rc::clone(&renderer_data.user_ctx().as_ref().unwrap().render_state);
    let now = window().unwrap().performance().unwrap().now();
    let render_textures: Vec<WebGlTexture> = renderer_data
        .textures_by_id([
            TextureId::PrevRender,
            TextureId::AveragedRenderA,
            TextureId::AveragedRenderB,
        ])
        .into_iter()
        .map(|texture| texture.webgl_texture().clone())
        .collect();

    render_state.borrow_mut().update_position();

    let should_render = {
        let render_state = render_state.borrow();
        !render_state.is_paused()
            || render_state.should_save_image()
            || render_state.render_count() == 0
    };

    let should_sync_dimensions = {
        let render_state = render_state.borrow();
        (render_state.window_size_out_of_sync()
            && now - render_state.prev_resize_sync_time() > RESIZE_UPDATE_DEBOUNCE_INTERVAL)
            || render_state.render_count() == 0
    };

    // debounce resize handler
    if should_sync_dimensions {
        let mut render_state = render_state.borrow_mut();
        render_state.sync_dimensions(gl, &render_textures, canvas, now);
    }

    if should_render {
        renderer_data.update_uniforms();
        renderer_data.render();
        render_state.borrow_mut().inc_render_count();

        // screenshots should be saved immediately after rendering
        {
            let mut render_state = render_state.borrow_mut();
            if render_state.should_save_image() {
                render_state.set_should_save_image(false);
                renderer_data.save_image();
            }
        }
    }
}
