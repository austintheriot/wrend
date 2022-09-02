use super::{
    attribute_id::AttributeId, buffer_id::BufferId, fragment_shader_id::FragmentShaderId,
    framebuffer_id::FramebufferId, program_id::ProgramId, texture_id::TextureId,
    transform_feedback_id::TransformFeedbackId, uniform_id::UniformId, vao_id::VAOId,
    vertex_shader_id::VertexShaderId,
};
use crate::{state::render_state_handle::RenderStateHandle, utils};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use wrend::RendererData;

// reusable draw call for both canvas and framebuffer
fn draw_quad(gl: &WebGl2RenderingContext, canvas: &HtmlCanvasElement) {
    // sync canvas dimensions with viewport
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    // clear canvas
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    // draw
    let primitive_type = WebGl2RenderingContext::TRIANGLES; // draws a triangle after shader is run every 3 times
    let offset = 0;
    let count = 6; // this will execute vertex shader 3 times
    gl.draw_arrays(primitive_type, offset, count);
}

pub fn render(
    renderer_data: &RendererData<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        TransformFeedbackId,
        VAOId,
        RenderStateHandle,
    >,
) {
    let gl = renderer_data.gl();
    let canvas = renderer_data.canvas();
    let user_ctx = renderer_data
        .user_ctx()
        .expect("RenderState should exist during render callback")
        .get();

    let num_particles = user_ctx.borrow().num_particles();

    // RETRIEVING ALL WEBGL OBJECTS --------------------------------------------------------
    let white_noise_texture = renderer_data
        .texture(&TextureId::WhiteNoise)
        .expect("WhiteNoiseTexture should exist in renderer_data")
        .webgl_texture();

    let perlin_noise_texture = renderer_data
        .texture(&TextureId::PerlinNoise)
        .expect("PerlinNoiseTexture should exist in renderer_data")
        .webgl_texture();

    let perlin_noise_framebuffer = renderer_data
        .framebuffer(&FramebufferId::PerlinNoise)
        .expect("PerlinNoise Framebuffer should exist in renderer_data")
        .webgl_framebuffer();

    // note: this mutates global state (so that on the next render, the buffers are swapped)
    let particle_read_write_buffer = user_ctx.borrow_mut().next_read_write_buffers();

    let (particle_read_buffer_id, update_particle_vao_read_id) =
        particle_read_write_buffer.read_ids();
    let particle_read_buffer = renderer_data
        .buffer(&particle_read_buffer_id)
        .expect("Particle read buffer should exist in renderer_data");
    let webgl_particle_read_buffer = particle_read_buffer.webgl_buffer();

    let (particle_write_buffer_id, _) = particle_read_write_buffer.write_ids();
    let particle_write_buffer = renderer_data
        .buffer(&particle_write_buffer_id)
        .expect("Particle write buffer should exist in renderer_data");
    let webgl_particle_write_buffer = particle_write_buffer.webgl_buffer();

    let transform_feedback = renderer_data
        .transform_feedback(&TransformFeedbackId::Particle)
        .expect("Transform feedback should exist in the renderer_data");

    // RENDER NEW PERLIN NOISE TO FRAMEBUFFER --------------------------------------------------------
    renderer_data.use_program(&ProgramId::PerlinNoise);
    renderer_data.use_vao(&VAOId::PerlinNoise);
    gl.active_texture(WebGl2RenderingContext::TEXTURE0);
    gl.bind_texture(
        WebGl2RenderingContext::TEXTURE_2D,
        Some(white_noise_texture),
    );
    gl.bind_framebuffer(
        WebGl2RenderingContext::FRAMEBUFFER,
        Some(perlin_noise_framebuffer),
    );
    draw_quad(gl, canvas);
    gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);

    // UPDATE PARTICLE POSITIONS --------------------------------------------------------
    renderer_data.use_program(&ProgramId::UpdateParticles);
    // must bind the VAO that corresponds to the read buffer (and not the write buffer)
    // because it is a WebGL error to bind to a buffer via a VAO and Transform Feedback at the same time
    renderer_data.use_vao(&update_particle_vao_read_id);
    gl.bind_buffer(
        WebGl2RenderingContext::ARRAY_BUFFER,
        Some(webgl_particle_read_buffer),
    );
    gl.active_texture(WebGl2RenderingContext::TEXTURE1);
    gl.bind_texture(
        WebGl2RenderingContext::TEXTURE_2D,
        Some(perlin_noise_texture),
    );

    gl.bind_transform_feedback(
        WebGl2RenderingContext::TRANSFORM_FEEDBACK,
        Some(transform_feedback),
    );
    gl.bind_buffer_base(
        WebGl2RenderingContext::TRANSFORM_FEEDBACK_BUFFER,
        0,
        Some(webgl_particle_write_buffer),
    );
    gl.enable(WebGl2RenderingContext::RASTERIZER_DISCARD);
    gl.begin_transform_feedback(WebGl2RenderingContext::POINTS);
    gl.draw_arrays(WebGl2RenderingContext::POINTS, 0, num_particles as i32);
    gl.disable(WebGl2RenderingContext::RASTERIZER_DISCARD);
    gl.end_transform_feedback();
    gl.bind_buffer_base(WebGl2RenderingContext::TRANSFORM_FEEDBACK_BUFFER, 0, None);
    gl.bind_transform_feedback(WebGl2RenderingContext::TRANSFORM_FEEDBACK, None);

    // DRAW PARTICLES TO CANVAS --------------------------------------------------------
    renderer_data.use_program(&ProgramId::DrawParticles);
    renderer_data.use_vao(&VAOId::DrawParticles);
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    if user_ctx.borrow().is_first_render() {
        user_ctx.borrow_mut().set_is_first_render(false);
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    gl.disable(WebGl2RenderingContext::DEPTH_TEST);
    gl.disable(WebGl2RenderingContext::CULL_FACE);
    gl.enable(WebGl2RenderingContext::BLEND);
    // good for adding light particle values together:
    gl.blend_func(
        WebGl2RenderingContext::SRC_ALPHA,
        WebGl2RenderingContext::ONE,
    );
    // good for adding dark particle values together:
    // gl.blend_func(
    //     WebGl2RenderingContext::ONE,
    //     WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
    // );
    gl.draw_arrays(WebGl2RenderingContext::POINTS, 0, num_particles as i32);

    if user_ctx.borrow().should_save_image() {
        user_ctx.borrow_mut().set_should_save_image(false);
        utils::save_image(canvas);
    }
}
