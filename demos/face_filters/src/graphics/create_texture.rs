use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlTexture};
use wrend::TextureCreateContext;

use crate::state::{RenderState, RenderStateHandle};

use super::TextureId;

pub fn make_crate_src_video_texture(
    render_state_handle: RenderStateHandle,
) -> impl Fn(&TextureCreateContext) -> WebGlTexture {
    move |ctx: &TextureCreateContext| {
        let gl = ctx.gl();
        let webgl_texture = gl
            .create_texture()
            .expect("Should be able to create textures from WebGL context");

        gl.active_texture(WebGl2RenderingContext::TEXTURE0 + TextureId::SrcVideo.location());
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

        let (width, height) = {
            let state_ref = render_state_handle.borrow();
            let src_video = state_ref.src_video();
            (src_video.width(), src_video.height())
        };

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

/// Creates a texture that will eventually hold a complete render from WebGL
pub fn make_create_render_texture(
    render_state_handle: RenderStateHandle,
    texture_id: TextureId,
) -> impl Fn(&TextureCreateContext) -> WebGlTexture {
    move |ctx: &TextureCreateContext| {
        let gl = ctx.gl();
        let webgl_texture = gl
            .create_texture()
            .expect("Should be able to create textures from WebGL context");

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

        let (width, height) = {
            let state_ref = render_state_handle.borrow();
            let src_video = state_ref.src_video();
            (src_video.width(), src_video.height())
        };

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
