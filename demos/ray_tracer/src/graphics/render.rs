use super::{
    attribute_id::AttributeId, buffer_id::BufferId, fragment_shader_id::FragmentShaderId,
    framebuffer_id::FramebufferId, program_id::ProgramId, texture_id::TextureId,
    transform_feedback_id::TransformFeedbackId, vao_id::VAOId,
    vertex_shader_id::VertexShaderId,
};
use crate::state::state_handle::StateHandle;
use web_sys::WebGl2RenderingContext;
use wrend::{Renderer, QUAD};

// reusable draw call for both canvas and framebuffer
fn draw_quad(gl: &WebGl2RenderingContext, width: i32, height: i32) {
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.viewport(0, 0, width, height);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    gl.draw_arrays(
        WebGl2RenderingContext::TRIANGLES,
        0,
        (QUAD.len() / 2) as i32,
    );
}

pub fn render(
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
        StateHandle,
    >,
) {
    let gl = renderer.gl();

    let app_state_ref = renderer.user_ctx().as_ref().unwrap().borrow();
    let render_state = app_state_ref.render_state();
    let width = render_state.width() as i32;
    let height = render_state.height() as i32;
    std::mem::drop(app_state_ref);

    // fow now, do a single draw directly to the canvas
    renderer.use_program(&ProgramId::RayTracer);
    renderer.use_vao(&VAOId::Quad);
    gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
    draw_quad(gl, width, height);

    // render raw render into framebuffer
    //  let prev_render_framebuffer = renderer
    //  .framebuffer(&FramebufferId::PrevRender)
    //  .map(|framebuffer| framebuffer.webgl_framebuffer());

    // render a combined prev_render & averaged_render into the other averaged_render texture

    // draw the new averaged_render onto the canvas
}
