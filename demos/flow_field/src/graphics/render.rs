use super::{
    buffer_id::BufferId, program_id::ProgramId, shader_id::ShaderId, texture_id::TextureId,
    uniform_id::UniformId,
};
use crate::state::render_state::RenderState;
use std::{cell::RefCell, rc::Rc};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use wrend::renderer::{default_id::DefaultId, renderer::Renderer};

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
        ShaderId,
        ShaderId,
        ProgramId,
        UniformId,
        BufferId,
        TextureId,
        DefaultId,
        Rc<RefCell<RenderState>>,
    >,
) {
    let gl = renderer.gl();
    let canvas = renderer.canvas();

    let program = renderer
        .programs()
        .get(&ProgramId::PassThrough)
        .expect("Should contain pass through program");
    gl.use_program(Some(program));

    let noise_texture = renderer
        .textures()
        .get(&TextureId::Noise)
        .map(|texture| texture.webgl_texture());
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, noise_texture);

    gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
    draw(gl, canvas);
}
