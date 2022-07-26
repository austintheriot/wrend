use super::{
    attribute_id::AttributeId, buffer_id::BufferId, fragment_shader_id::FragmentShaderId,
    framebuffer_id::FramebufferId, program_id::ProgramId, texture_id::TextureId,
    transform_feedback_id::TransformFeedbackId, vao_id::VAOId, vertex_shader_id::VertexShaderId,
};
use crate::state::app_context::AppContext;
use web_sys::WebGl2RenderingContext;
use wrend::{Renderer, QUAD};

// reusable draw call for both canvas and framebuffer
fn draw_quad(gl: &WebGl2RenderingContext) {
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
        AppContext,
    >,
) {
    let gl = renderer.gl();

    // fow now, do a single draw directly to the canvas
    renderer.use_program(&ProgramId::RayTracer);
    renderer.use_vao(&VAOId::Quad);
    gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
    draw_quad(gl);

    // render raw render into framebuffer
    //  let prev_render_framebuffer = renderer
    //  .framebuffer(&FramebufferId::PrevRender)
    //  .map(|framebuffer| framebuffer.webgl_framebuffer());

    // render a combined prev_render & averaged_render into the other averaged_render texture

    // draw the new averaged_render onto the canvas
}
