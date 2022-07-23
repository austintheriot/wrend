use web_sys::WebGl2RenderingContext;
use wrend::AttributeCreateContext;

pub fn create_position_attribute<UserCtx: Clone>(ctx: &AttributeCreateContext<UserCtx>) {
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
}
