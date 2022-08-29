use web_sys::WebGl2RenderingContext;
use wrend::AttributeCreateContext;

pub fn create_quad_vertex_attribute(ctx: &AttributeCreateContext) {
    let gl = ctx.gl();
    let webgl_buffer = ctx.webgl_buffer();
    let attribute_location = ctx.attribute_location();

    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(webgl_buffer));
    gl.vertex_attrib_pointer_with_i32(
        attribute_location.into(),
        2,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
}

pub fn create_particle_position_attribute(ctx: &AttributeCreateContext) {
    let gl = ctx.gl();
    let webgl_buffer = ctx.webgl_buffer();

    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(webgl_buffer));
    gl.vertex_attrib_pointer_with_i32(
        ctx.attribute_location().into(),
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
}

pub fn create_particle_color_attribute(ctx: &AttributeCreateContext) {
    let gl = ctx.gl();
    let webgl_buffer = ctx.webgl_buffer();

    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(webgl_buffer));
    gl.vertex_attrib_pointer_with_i32(
        ctx.attribute_location().into(),
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
}
