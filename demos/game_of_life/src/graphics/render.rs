use super::{
    buffer_id::BufferId, framebuffer_id::FramebufferId, program_id::ProgramId, shader_id::ShaderId,
    texture_id::TextureId, uniform_id::UniformId,
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use webgl::renderer::renderer::Renderer;
use yew::UseStateHandle;

pub fn render(
    renderer: &Renderer<
        ShaderId,
        ShaderId,
        ProgramId,
        UniformId,
        BufferId,
        TextureId,
        FramebufferId,
        UseStateHandle<i32>,
    >,
) {
    let gl = renderer.gl();
    let canvas: HtmlCanvasElement = gl.canvas().unwrap().dyn_into().unwrap();

    // use the appropriate program
    gl.use_program(renderer.programs().get(&ProgramId));

    // draw to canvas (instead of framebuffer)
    gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);

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
