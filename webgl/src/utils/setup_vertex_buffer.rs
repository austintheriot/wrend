use js_sys::Float32Array;
use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

pub fn setup_vertex_buffer(
    gl: &WebGl2RenderingContext,
    program: &WebGlProgram,
    vertex_array: &Float32Array,
) -> Result<(), JsValue> {
    let vertex_attribute_position = gl.get_attrib_location(program, "a_position") as u32;
    let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        vertex_array,
        WebGl2RenderingContext::STATIC_DRAW,
    );
    gl.enable_vertex_attrib_array(vertex_attribute_position);
    gl.vertex_attrib_pointer_with_i32(
        vertex_attribute_position,
        2,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );

    Ok(())
}
