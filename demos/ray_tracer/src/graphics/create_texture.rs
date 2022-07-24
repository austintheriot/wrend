use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlTexture};
use wrend::TextureCreateContext;

pub fn create_ray_tracer_texture<UserCtx>(ctx: &TextureCreateContext<UserCtx>) -> WebGlTexture {
    let gl = ctx.gl();
    let webgl_texture = gl
        .create_texture()
        .expect("Should be able to create textures from WebGL context");
    let canvas: HtmlCanvasElement = gl.canvas().unwrap().dyn_into().unwrap();

    gl.active_texture(WebGl2RenderingContext::TEXTURE1);
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&webgl_texture));

    // Set the parameters so we don't need mips, we're not filtering, and we don't repeat
    gl.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_WRAP_S,
        WebGl2RenderingContext::REPEAT as i32,
    );
    gl.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_WRAP_T,
        WebGl2RenderingContext::REPEAT as i32,
    );
    gl.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_MIN_FILTER,
        WebGl2RenderingContext::NEAREST as i32,
    );
    gl.tex_parameteri(
        WebGl2RenderingContext::TEXTURE_2D,
        WebGl2RenderingContext::TEXTURE_MAG_FILTER,
        WebGl2RenderingContext::NEAREST as i32,
    );

    // use blank texture for now, since this will be filled with data as a separate render
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        WebGl2RenderingContext::TEXTURE_2D,
        0,
        WebGl2RenderingContext::RGBA as i32,
        canvas.width() as i32,
        canvas.height() as i32,
        0,
        WebGl2RenderingContext::RGBA,
        WebGl2RenderingContext::UNSIGNED_BYTE,
        None,
    )
    .unwrap();

    webgl_texture
}
