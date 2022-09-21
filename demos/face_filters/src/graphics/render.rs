use super::{
    attribute_id::AttributeId, buffer_id::BufferId, fragment_shader_id::FragmentShaderId,
    framebuffer_id::FramebufferId, program_id::ProgramId, texture_id::TextureId,
    transform_feedback_id::TransformFeedbackId, uniform_id::UniformId,
    vertex_shader_id::VertexShaderId, VAOId,
};
use crate::state::RenderStateHandle;
use log::{error, info};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use wrend::RendererData;

// reusable draw call for both canvas and framebuffer
fn draw(gl: &WebGl2RenderingContext, canvas: &HtmlCanvasElement) {
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
    gl.clear_color(0.0, 0.0, 0.0, 0.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);
}

pub fn render(
    renderer_data: &RendererData<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        UniformId,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        TransformFeedbackId,
        VAOId,
        RenderStateHandle,
    >,
) {
    let gl = renderer_data.gl();
    let canvas = renderer_data.canvas();
    let render_state_handle = renderer_data.user_ctx().unwrap();
    let src_video_element = render_state_handle.borrow().src_video().clone();
    let src_video_texture = renderer_data
    .texture(&TextureId::SrcVideo)
    .unwrap()
    .webgl_texture();

    let src_video_width = src_video_element.video_width();
    let src_video_height = src_video_element.video_height();

    // adjust canvas to match video element size
    if canvas.width() != src_video_width {
        info!("Canvas width is {}", canvas.width());
        info!("Setting width {src_video_width}");
        canvas.set_width(src_video_width)
    }
    if canvas.height() != src_video_height {
        info!("Canvas height is {}", canvas.height());
        info!("Setting height {src_video_height}");
        canvas.set_height(src_video_height)
    }

    // upload video data as texture
    if src_video_width > 0 && src_video_height > 0 {
        gl.active_texture(WebGl2RenderingContext::TEXTURE0 + TextureId::SrcVideo.location());
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(src_video_texture));
        if let Err(err) = gl.tex_image_2d_with_u32_and_u32_and_html_video_element(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            &src_video_element,
        ) {
            error!("Error uploading src video as a WebGL texture: {:?}", err);
        }
    }

    renderer_data.use_program(&ProgramId::Unfiltered);
    renderer_data.use_vao(&VAOId::Quad);
    gl.active_texture(WebGl2RenderingContext::TEXTURE0 + TextureId::SrcVideo.location());
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(src_video_texture));
    gl.bind_framebuffer(
        WebGl2RenderingContext::FRAMEBUFFER,
        None,
    );
    draw(gl, canvas);
}
