use web_sys::{WebGl2RenderingContext, WebGlBuffer};
use wrend::{BufferCreateContext, QUAD};

pub fn create_vertex_buffer<UserCtx>(ctx: &BufferCreateContext<UserCtx>) -> WebGlBuffer {
    let gl = ctx.gl();
    let buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    // requires `unsafe` since we're creating a raw view into wasm memory,
    // but this array is static, so it shouldn't cause any issues
    let vertex_array = unsafe { js_sys::Float32Array::view(&QUAD) };
    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &vertex_array,
        WebGl2RenderingContext::STATIC_DRAW,
    );

    buffer
}
