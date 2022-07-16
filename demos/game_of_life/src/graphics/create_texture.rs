use js_sys::Math;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlTexture};
use wrend::renderer::texture_create_context::TextureCreateContext;

pub fn create_texture<UserCtx>(ctx: TextureCreateContext<UserCtx>) -> WebGlTexture {
    let gl = ctx.gl();
    let webgl_texture = gl
        .create_texture()
        .expect("Should be able to create textures from WebGL context");
    let canvas: HtmlCanvasElement = gl.canvas().unwrap().dyn_into().unwrap();

    gl.active_texture(WebGl2RenderingContext::TEXTURE0);

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

    let canvas_width = canvas.width() as i32;
    let canvas_height = canvas.height() as i32;

    // generate a texture of noise
    let bytes_per_pixel = 4;
    let length_of_noise_array = (canvas_width * canvas_height * bytes_per_pixel) as usize;
    let mut noise_image = Vec::with_capacity(length_of_noise_array);
    for _ in 0..length_of_noise_array {
        noise_image.push(0u8);
    }
    for rgba in noise_image.chunks_mut(bytes_per_pixel as usize) {
        let random_float = Math::random();
        let black_or_white = if random_float < 0.5 { u8::MIN } else { u8::MAX };

        rgba[0] = black_or_white;
        rgba[1] = black_or_white;
        rgba[2] = black_or_white;
        rgba[3] = u8::MAX;
    }

    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_u8_array_and_src_offset(
        WebGl2RenderingContext::TEXTURE_2D,
        0,
        WebGl2RenderingContext::RGBA as i32,
        canvas.width() as i32,
        canvas.height() as i32,
        0,
        WebGl2RenderingContext::RGBA,
        WebGl2RenderingContext::UNSIGNED_BYTE,
        &noise_image,
        0,
    )
    .unwrap();

    webgl_texture
}
