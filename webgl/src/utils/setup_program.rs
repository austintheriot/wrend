use super::compile_shader::compile_shader;
use super::link_program::link_program;
use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

pub async fn setup_program(
    gl: &WebGl2RenderingContext,
    vertex_shader_src: &str,
    fragment_shader_src: &str,
) -> Result<WebGlProgram, JsValue> {
    let vertex_shader = compile_shader(
        gl,
        WebGl2RenderingContext::VERTEX_SHADER,
        &vertex_shader_src,
    )?;
    let fragment_shader = compile_shader(
        gl,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        &fragment_shader_src,
    )?;
    let program = link_program(gl, &vertex_shader, &fragment_shader)?;
    gl.use_program(Some(&program));

    Ok(program)
}
