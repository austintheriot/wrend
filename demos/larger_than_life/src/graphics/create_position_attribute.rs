use web_sys::{WebGl2RenderingContext, WebGlBuffer};
use wrend::{
    constants::quad::QUAD,
    renderer::{
        attribute_create_context::AttributeCreateContext,
        buffer_create_context::AttributeCreateContext,
    },
};

pub fn create_position_attribute<UserCtx>(ctx: AttributeCreateContext<UserCtx>) {
    let gl = ctx.gl();
    let webgl_buffer = ctx.webgl_buffer();
    let attribute_location = ctx.attribute_location();

    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&webgl_buffer));
    gl.vertex_attrib_pointer_with_i32(
        attribute_location.into(),
        2,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );

    buffer
}
