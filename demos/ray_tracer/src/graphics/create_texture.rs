use super::texture_id::TextureId;
use crate::state::app_context::AppContext;

use web_sys::{WebGl2RenderingContext, WebGlTexture};
use wrend::TextureCreateContext;

/// Creates a texture that will eventually hold a complete render from WebGL
pub fn make_create_render_texture(
    texture_id: TextureId,
) -> impl Fn(&TextureCreateContext<AppContext>) -> WebGlTexture {
    move |ctx: &TextureCreateContext<AppContext>| {
        let gl = ctx.gl();
        let webgl_texture = gl
            .create_texture()
            .expect("Should be able to create textures from WebGL context");

        let render_state = ctx.user_ctx().as_ref().unwrap().render_state.borrow();
        let pipeline = render_state.camera();
        let width = pipeline.width();
        let height = pipeline.height();
        std::mem::drop(render_state);

        gl.active_texture(WebGl2RenderingContext::TEXTURE0 + texture_id.location());
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&webgl_texture));

        // Set the parameters so we don't need mips, we're not filtering, and we don't repeat
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );

        // load empty texture into gpu -- this will get rendered into later
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            width as i32,
            height as i32,
            0,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            None,
        )
        .unwrap();

        webgl_texture
    }
}
