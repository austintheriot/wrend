use super::{
    attribute_id::AttributeId, buffer_id::BufferId, fragment_shader_id::FragmentShaderId,
    framebuffer_id::FramebufferId, program_id::ProgramId, texture_id::TextureId,
    transform_feedback_id::TransformFeedbackId, uniform_id::UniformId,
    vertex_shader_id::VertexShaderId,
};
use crate::state::render_state_handle::RenderStateHandle;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use wrend::Renderer;

// reusable draw call for both canvas and framebuffer
fn draw(gl: &WebGl2RenderingContext, canvas: &HtmlCanvasElement) {
    // sync canvas dimensions with viewport
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    // clear canvas
    gl.clear_color(0.0, 0.0, 0.0, 0.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    // draw
    let primitive_type = WebGl2RenderingContext::TRIANGLES; // draws a triangle after shader is run every 3 times
    let offset = 0;
    let count = 6; // this will execute vertex shader 3 times
    gl.draw_arrays(primitive_type, offset, count);
}

pub fn render(
    renderer: &Renderer<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        TransformFeedbackId,
        ProgramId,
        RenderStateHandle,
    >,
) {
    let gl = renderer.gl();
    let canvas = renderer.canvas();

    // render perlin noise to framebuffer
    let white_noise_texture = renderer
        .textures()
        .get(&TextureId::WhiteNoise)
        .map(|texture| texture.webgl_texture());
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, white_noise_texture);
    renderer.use_program(&ProgramId::PerlinNoise);
    renderer.use_vao(&ProgramId::PerlinNoise);
    let perlin_noise_framebuffer = renderer
        .framebuffers()
        .get(&FramebufferId::PerlinNoise)
        .map(|framebuffer| framebuffer.webgl_framebuffer());
    gl.bind_framebuffer(
        WebGl2RenderingContext::FRAMEBUFFER,
        perlin_noise_framebuffer,
    );
    draw(gl, canvas);

    // copy perlin noise from framebuffer to canvas 
    // (this step could be replaced with a true render call,
    // where the perlin noise is used as a texture in the render)
    renderer.use_program(&ProgramId::PassThrough);
    renderer.use_vao(&ProgramId::PassThrough);
    let perlin_noise_texture = renderer
        .textures()
        .get(&TextureId::PerlinNoise)
        .map(|texture| texture.webgl_texture());
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, perlin_noise_texture);
    gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
    draw(gl, canvas);
}
