use super::{
    attribute_id::AttributeId, buffer_id::BufferId, fragment_shader_id::FragmentShaderId,
    framebuffer_id::FramebufferId, program_id::ProgramId, texture_id::TextureId,
    texture_id_number::TextureIdNumber, transform_feedback_id::TransformFeedbackId,
    uniform_id::UniformId, vertex_shader_id::VertexShaderId,
};
use crate::state::render_state_handle::RenderStateHandle;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use wrend::Renderer;

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
        RenderStateHandle,
    >,
) {
    let gl = renderer.gl();
    let canvas = renderer.canvas();
    let user_ctx = renderer
        .user_ctx()
        .expect("RenderState should exist during render callback")
        .get();

    let num_particles = user_ctx.borrow().num_particles();

    // RETRIEVING ALL WEBGL OBJECTS --------------------------------------------------------
    let white_noise_texture = renderer
        .textures()
        .get(&TextureId::WhiteNoise)
        .expect("WhiteNoiseTexture should exist in renderer")
        .webgl_texture();

    let perlin_noise_texture = renderer
        .textures()
        .get(&TextureId::PerlinNoise)
        .expect("PerlinNoiseTexture should exist in renderer")
        .webgl_texture();

    let perlin_noise_framebuffer = renderer
        .framebuffers()
        .get(&FramebufferId::PerlinNoise)
        .expect("PerlinNoise Framebuffer should exist in renderer")
        .webgl_framebuffer();

    let prev_frame_a_texture = renderer
        .textures()
        .get(&TextureId::PrevFrameA)
        .expect("PrevFrameA Texture should exist in renderer")
        .webgl_texture();

    let prev_frame_a_framebuffer = renderer
        .framebuffers()
        .get(&FramebufferId::PrevFrameA)
        .expect("PrevFrameA Framebuffer should exist in renderer")
        .webgl_framebuffer();

    // note: this mutates global state (so that on the next render, the buffers are swapped)
    let particle_read_write_buffer = user_ctx.borrow_mut().next_read_write_buffers();

    let particle_read_buffer_id = particle_read_write_buffer.read_buffer();
    let particle_read_buffer = renderer
        .buffers()
        .get(&particle_read_buffer_id)
        .expect("Particle read buffer should exist in renderer");
    let webgl_particle_read_buffer = particle_read_buffer.webgl_buffer();

    let particle_write_buffer_id = particle_read_write_buffer.write_buffer();
    let particle_write_buffer = renderer
        .buffers()
        .get(&particle_write_buffer_id)
        .expect("Particle write buffer should exist in renderer");
    let webgl_particle_write_buffer = particle_write_buffer.webgl_buffer();

    let transform_feedback = renderer
        .transform_feedbacks()
        .get(&TransformFeedbackId::Particle)
        .expect("Transform feedback should exist in the renderer");

    // RENDER NEW PERLIN NOISE TO FRAMEBUFFER --------------------------------------------------------
    renderer.switch_program(&ProgramId::PerlinNoise);
    // perlin noise samples from the static white noise texture for pseudo-randomness
    gl.active_texture(WebGl2RenderingContext::TEXTURE0 + TextureId::WhiteNoise.num());
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

    // // DEBUGGING PERLIN_NOISE --------------------------------------------------------
    //  pull from the framebuffer just drawn to and copy to the canvas
    renderer.switch_program(&ProgramId::DebugPerlinNoise);
    gl.active_texture(WebGl2RenderingContext::TEXTURE0 + TextureId::PerlinNoise.num());
    gl.bind_texture(
        WebGl2RenderingContext::TEXTURE_2D,
        Some(perlin_noise_texture),
    );
    gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
    draw_quad(gl, canvas);

    // // // UPDATE PARTICLE POSITIONS --------------------------------------------------------
    // // // ***NOTE***: can't use VAO here (i.e. renderer.switch_program()), since a buffer can't simultaneously
    // // // be bound to a VAO and also a transform feedback at the same time
    // let program = renderer
    //     .programs()
    //     .get(&ProgramId::UpdateParticles)
    //     .unwrap();
    // gl.use_program(Some(program));
    // gl.active_texture(WebGl2RenderingContext::TEXTURE0 + TextureId::PerlinNoise.num());
    // gl.bind_texture(
    //     WebGl2RenderingContext::TEXTURE_2D,
    //     Some(perlin_noise_texture),
    // );
    // gl.bind_buffer(
    //     WebGl2RenderingContext::ARRAY_BUFFER,
    //     Some(webgl_particle_read_buffer),
    // );
    // gl.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);

    // gl.bind_transform_feedback(
    //     WebGl2RenderingContext::TRANSFORM_FEEDBACK,
    //     Some(transform_feedback),
    // );
    // gl.bind_buffer_base(
    //     WebGl2RenderingContext::TRANSFORM_FEEDBACK_BUFFER,
    //     0,
    //     Some(webgl_particle_write_buffer),
    // );
    // gl.enable(WebGl2RenderingContext::RASTERIZER_DISCARD);
    // gl.begin_transform_feedback(WebGl2RenderingContext::POINTS);
    // gl.draw_arrays(WebGl2RenderingContext::POINTS, 0, num_particles as i32);
    // gl.disable(WebGl2RenderingContext::RASTERIZER_DISCARD);
    // gl.end_transform_feedback();
    // gl.bind_buffer_base(WebGl2RenderingContext::TRANSFORM_FEEDBACK_BUFFER, 0, None);
    // gl.bind_transform_feedback(WebGl2RenderingContext::TRANSFORM_FEEDBACK, None);

    // // DRAW PARTICLES TO FRAMEBUFFER HOLDING THE PREV FRAME --------------------------------------------------------
    // let program = renderer.programs().get(&ProgramId::DrawParticles).unwrap();
    // gl.use_program(Some(program));

    // gl.disable(WebGl2RenderingContext::DEPTH_TEST);
    // gl.disable(WebGl2RenderingContext::CULL_FACE);
    // gl.blend_func(
    //     WebGl2RenderingContext::SRC_ALPHA,
    //     WebGl2RenderingContext::ONE,
    // );
    // gl.enable(WebGl2RenderingContext::BLEND);
    // gl.bind_buffer(
    //     WebGl2RenderingContext::ARRAY_BUFFER,
    //     Some(webgl_particle_write_buffer),
    // );
    // gl.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    // gl.bind_framebuffer(
    //     WebGl2RenderingContext::FRAMEBUFFER,
    //     Some(prev_frame_a_framebuffer),
    // );
    // gl.draw_arrays(WebGl2RenderingContext::POINTS, 0, num_particles as i32);

    // // AVERAGE PREV FRAMES AND DRAW TO CANVAS --------------------------------------------------------
    // renderer.switch_program(&ProgramId::PassThrough);
    // gl.active_texture(WebGl2RenderingContext::TEXTURE0 + TextureId::PrevFrameA.num());
    // gl.bind_texture(
    //     WebGl2RenderingContext::TEXTURE_2D,
    //     Some(prev_frame_a_texture),
    // );
    // gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);

    // gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
    // gl.clear_color(0.0, 0.0, 0.0, 1.0);
    // gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    // draw_quad(gl, canvas);
}
