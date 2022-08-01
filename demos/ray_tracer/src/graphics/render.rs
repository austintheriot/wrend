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
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
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
        AppContext,
    >,
) {
    let gl = renderer.gl();
    let user_ctx = renderer.user_ctx().unwrap();

    let write_averaged_render_framebuffer_id: FramebufferId;
    let write_averaged_render_texture_id: TextureId;
    let read_averaged_render_texture_id: TextureId;

    if user_ctx.render_state.borrow().render_count() % 2 == 0 {
        write_averaged_render_framebuffer_id = FramebufferId::AveragedRenderA;
        write_averaged_render_texture_id = TextureId::AveragedRenderA;
        read_averaged_render_texture_id = TextureId::AveragedRenderB;
    } else {
        write_averaged_render_framebuffer_id = FramebufferId::AveragedRenderB;
        write_averaged_render_texture_id = TextureId::AveragedRenderB;
        read_averaged_render_texture_id = TextureId::AveragedRenderA;
    };

    let write_prev_render_framebuffer = renderer
        .framebuffer(&FramebufferId::PrevRender)
        .unwrap()
        .webgl_framebuffer();
    let read_prev_render_texture = renderer
        .texture(&TextureId::PrevRender)
        .unwrap()
        .webgl_texture();
    let write_averaged_render_framebuffer = renderer
        .framebuffer(&write_averaged_render_framebuffer_id)
        .unwrap()
        .webgl_framebuffer();
    let write_averaged_render_texture = renderer
        .texture(&write_averaged_render_texture_id)
        .unwrap()
        .webgl_texture();
    let read_averaged_render_texture = renderer
        .texture(&read_averaged_render_texture_id)
        .unwrap()
        .webgl_texture();

    renderer.use_vao(&VAOId::Quad);

    // render a plain, raw render into the prev_render framebuffer
    // no textures are necessary for this operation
    renderer.use_program(&ProgramId::RayTracer);
    gl.bind_framebuffer(
        WebGl2RenderingContext::FRAMEBUFFER,
        Some(write_prev_render_framebuffer),
    );
    draw_quad(gl);

    // pull the raw, previous render texture, and combine with a PREVIOUS averaged render texture,
    // rendering into a NEW averaged render framebuffer
    // (if the render count is < 2, then the prev_render is just drawn to the framebuffer)
    renderer.use_program(&ProgramId::AverageRenders);
    gl.active_texture(WebGl2RenderingContext::TEXTURE0 + TextureId::PrevRender.location());
    gl.bind_texture(
        WebGl2RenderingContext::TEXTURE_2D,
        Some(read_prev_render_texture),
    );
    gl.active_texture(
        WebGl2RenderingContext::TEXTURE0 + read_averaged_render_texture_id.location(),
    );
    gl.bind_texture(
        WebGl2RenderingContext::TEXTURE_2D,
        Some(read_averaged_render_texture),
    );
    gl.bind_framebuffer(
        WebGl2RenderingContext::FRAMEBUFFER,
        Some(write_averaged_render_framebuffer),
    );
    draw_quad(gl);

    // copy the just drawn-to, averaged render onto the canvas
    // (if the render count is < 2, then the prev_render is just drawn to the canvas)
    renderer.use_program(&ProgramId::PassThrough);
    gl.active_texture(WebGl2RenderingContext::TEXTURE0 + TextureId::PrevRender.location());
    gl.bind_texture(
        WebGl2RenderingContext::TEXTURE_2D,
        Some(read_prev_render_texture),
    );
    gl.active_texture(
        WebGl2RenderingContext::TEXTURE0 + write_averaged_render_texture_id.location(),
    );
    gl.bind_texture(
        WebGl2RenderingContext::TEXTURE_2D,
        Some(write_averaged_render_texture),
    );
    gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
    draw_quad(gl);
}
