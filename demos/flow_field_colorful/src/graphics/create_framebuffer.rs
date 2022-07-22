use crate::state::render_state_handle::RenderStateHandle;
use web_sys::{WebGl2RenderingContext, WebGlFramebuffer};
use wrend::FramebufferCreateContext;

pub fn create_perlin_noise_framebuffer(
    ctx: &FramebufferCreateContext<RenderStateHandle>,
) -> WebGlFramebuffer {
    let texture_a = ctx
        .webgl_texture()
        .as_ref()
        .expect("RendererBuilder should have NoiseTexture built when creating framebuffers");
    let gl = ctx.gl();

    let framebuffer_object = gl
        .create_framebuffer()
        .expect("WebGL2 should be able to create a WebGlFramebuffer object");
    gl.bind_framebuffer(
        WebGl2RenderingContext::FRAMEBUFFER,
        Some(&framebuffer_object),
    );
    gl.framebuffer_texture_2d(
        WebGl2RenderingContext::FRAMEBUFFER,
        WebGl2RenderingContext::COLOR_ATTACHMENT0,
        WebGl2RenderingContext::TEXTURE_2D,
        Some(texture_a),
        0,
    );
    gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);

    framebuffer_object
}
