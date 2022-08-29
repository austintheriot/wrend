use crate::state::render_state_handle::RenderStateHandle;
use js_sys::Math;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};
use wrend::{BufferCreateContext, QUAD};

pub fn create_quad_vertex_buffer(ctx: &BufferCreateContext) -> WebGlBuffer {
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

    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

    buffer
}

pub fn make_create_particle_buffer_a(
    app_context: RenderStateHandle,
) -> impl Fn(&BufferCreateContext) -> WebGlBuffer {
    move |ctx| {
        let gl = ctx.gl();
        let buffer = gl.create_buffer().unwrap();
        let num_particle_vertices = app_context.get().borrow().num_particle_vertices();

        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        let initial_particle_positions: Vec<f32> = vec![0.0; num_particle_vertices as usize]
            .into_iter()
            .map(|_| (Math::random() * 2.0 - 1.0) as f32)
            .collect();
        let initial_particle_positions =
            unsafe { js_sys::Float32Array::view(&initial_particle_positions) };
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &initial_particle_positions,
            WebGl2RenderingContext::DYNAMIC_COPY,
        );

        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

        buffer
    }
}

pub fn make_create_particle_buffer_b(
    app_context: RenderStateHandle,
) -> impl Fn(&BufferCreateContext) -> WebGlBuffer {
    move |ctx| {
        let gl = ctx.gl();
        let buffer = gl.create_buffer().unwrap();
        let num_particle_vertices = app_context.get().borrow().num_particle_vertices();

        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        let empty_particle_position_buffer =
            unsafe { js_sys::Float32Array::view(&vec![0.0; num_particle_vertices as usize]) };
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &empty_particle_position_buffer,
            WebGl2RenderingContext::DYNAMIC_COPY,
        );

        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

        buffer
    }
}
