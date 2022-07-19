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

    // RENDER NEW PERLIN NOISE TO FRAMEBUFFER --------------------------------------------------------
    let quad_vertex_buffer = renderer
        .buffers()
        .get(&BufferId::QuadVertexBuffer)
        .expect("QuadVertexBuffer should exist in renderer")
        .webgl_buffer();
    gl.bind_buffer(
        WebGl2RenderingContext::ARRAY_BUFFER,
        Some(quad_vertex_buffer),
    );
    let white_noise_texture = renderer
        .textures()
        .get(&TextureId::WhiteNoise)
        .expect("WhiteNoiseTexture should exist in renderer")
        .webgl_texture();
    gl.bind_texture(
        WebGl2RenderingContext::TEXTURE_2D,
        Some(white_noise_texture),
    );
    renderer.switch_program(&ProgramId::PerlinNoise);
    let perlin_noise_framebuffer = renderer
        .framebuffers()
        .get(&FramebufferId::PerlinNoise)
        .expect("PerlinNoise Framebuffer should exist in renderer")
        .webgl_framebuffer();
    gl.bind_framebuffer(
        WebGl2RenderingContext::FRAMEBUFFER,
        Some(perlin_noise_framebuffer),
    );
    draw(gl, canvas);
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
    gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
    
    // UPDATE PARTICLE POSITIONS --------------------------------------------------------

    // ***NOTE***: can't use VAO here (i.e. renderer.switch_program()), since a buffer can't simultaneously
    // be bound to a VAO and also a transform feedback at the same time 
    let program = renderer.programs().get(&ProgramId::UpdateParticles).unwrap();
    gl.use_program(Some(program));
    let read_write_buffer = user_ctx.borrow_mut().next_read_write_buffers();
    let read_buffer_id = read_write_buffer.read_buffer();
    let read_buffer = renderer
        .buffers()
        .get(&read_buffer_id)
        .expect("Read buffer should exist in renderer");
    let webgl_read_buffer = read_buffer.webgl_buffer();
    gl.bind_buffer(
        WebGl2RenderingContext::ARRAY_BUFFER,
        Some(webgl_read_buffer),
    );
    gl.vertex_attrib_pointer_with_i32(
        0,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );

    let transform_feedback = renderer
        .transform_feedbacks()
        .get(&TransformFeedbackId::Particle)
        .expect("Transform feedback should exist in the renderer");
    gl.bind_transform_feedback(
        WebGl2RenderingContext::TRANSFORM_FEEDBACK,
        Some(transform_feedback),
    );

    let write_buffer_id = read_write_buffer.write_buffer();
    let write_buffer = renderer
        .buffers()
        .get(&write_buffer_id)
        .expect("Write buffer should exist in renderer");
    let webgl_write_buffer = write_buffer.webgl_buffer();
    gl.bind_buffer_base(
        WebGl2RenderingContext::TRANSFORM_FEEDBACK_BUFFER,
        0,
        Some(webgl_write_buffer),
    );

    gl.enable(WebGl2RenderingContext::RASTERIZER_DISCARD);
    gl.begin_transform_feedback(WebGl2RenderingContext::POINTS);
    gl.draw_arrays(WebGl2RenderingContext::POINTS, 0, num_particles as i32);
    gl.disable(WebGl2RenderingContext::RASTERIZER_DISCARD);
    gl.end_transform_feedback();
    gl.bind_buffer_base(WebGl2RenderingContext::TRANSFORM_FEEDBACK_BUFFER, 0, None);
    gl.bind_transform_feedback(WebGl2RenderingContext::TRANSFORM_FEEDBACK, None);

    // DRAW PARTICLES TO CANVAS --------------------------------------------------------
    let program = renderer.programs().get(&ProgramId::DrawParticles).unwrap();
    gl.use_program(Some(program));
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    gl.disable(WebGl2RenderingContext::DEPTH_TEST);
    gl.disable(WebGl2RenderingContext::CULL_FACE);
    gl.blend_func(
        WebGl2RenderingContext::SRC_ALPHA,
        WebGl2RenderingContext::ONE,
    );
    gl.enable(WebGl2RenderingContext::BLEND);
    gl.bind_buffer(
        WebGl2RenderingContext::ARRAY_BUFFER,
        Some(webgl_write_buffer),
    );
    gl.vertex_attrib_pointer_with_i32(
        0,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );

    gl.draw_arrays(WebGl2RenderingContext::POINTS, 0, num_particles as i32);
}
