use super::{
    attribute_id::AttributeId, buffer_id::BufferId, fragment_shader_id::FragmentShaderId,
    framebuffer_id::FramebufferId, program_id::ProgramId, texture_id::TextureId,
    uniform_id::UniformId, vertex_shader_id::VertexShaderId,
};
use crate::state::render_state::RenderState;
use std::{cell::RefCell, rc::Rc};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use wrend::{IdDefault, Renderer};

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
        IdDefault,
        ProgramId,
        Rc<RefCell<RenderState>>,
    >,
) {
    // get current count from state and update it
    let user_ctx = renderer
        .user_ctx()
        .expect("Should have user_ctx available in render");
    let current_count = user_ctx.borrow().count();
    user_ctx.borrow_mut().inc_count();

    let gl = renderer.gl();
    let canvas = renderer.canvas();

    // use the appropriate program
    renderer.use_program(&ProgramId::GameOfLife);
    renderer.use_vao(&ProgramId::GameOfLife);

    // sample from texture previously rendered to
    // and render to the opposite framebuffer
    let (previous_texture_id, next_frame_buffer_id, next_texture_id) = if current_count % 2 == 0 {
        (TextureId::A, FramebufferId::B, TextureId::B)
    } else {
        (TextureId::B, FramebufferId::A, TextureId::A)
    };
    let previous_webgl_texture = renderer
        .textures()
        .get(&previous_texture_id)
        .map(|texture| texture.webgl_texture());
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, previous_webgl_texture);

    // render to framebuffer
    let next_frame_buffer = renderer
        .framebuffers()
        .get(&next_frame_buffer_id)
        .map(|framebuffer| framebuffer.webgl_framebuffer());
    gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, next_frame_buffer);
    draw(gl, canvas);

    // pull from the framebuffer just drawn to and copy to the canvas
    renderer.use_program(&ProgramId::PassThrough);
    renderer.use_vao(&ProgramId::PassThrough);

    let next_webgl_texture = renderer
        .textures()
        .get(&next_texture_id)
        .map(|texture| texture.webgl_texture());
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, next_webgl_texture);

    gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
    draw(gl, canvas);
}
