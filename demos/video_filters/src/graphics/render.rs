use super::{
    attribute_id::AttributeId, buffer_id::BufferId, fragment_shader_id::FragmentShaderId,
    framebuffer_id::FramebufferId, program_id::ProgramId, texture_id::TextureId,
    transform_feedback_id::TransformFeedbackId, vertex_shader_id::VertexShaderId, FilterType,
    VAOId,
};
use crate::state::RenderStateHandle;
use log::error;
use web_sys::{HtmlCanvasElement, HtmlVideoElement, WebGl2RenderingContext, WebGlTexture};
use wrend::RendererData;

/// Reusable draw call for multiple filter types
fn draw(gl: &WebGl2RenderingContext, canvas: &HtmlCanvasElement) {
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
    gl.clear_color(0.0, 0.0, 0.0, 0.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);
}

pub(crate) struct DataForRendering<'a> {
    renderer_data: &'a RendererData<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        String,
        BufferId,
        AttributeId,
        TextureId,
        FramebufferId,
        TransformFeedbackId,
        VAOId,
        RenderStateHandle,
    >,
    gl: &'a WebGl2RenderingContext,
    canvas: &'a HtmlCanvasElement,
    src_video_texture: &'a WebGlTexture,
}

/// Renders using the Unfiltered filter
pub(crate) fn render_unfiltered(
    DataForRendering {
        canvas,
        renderer_data,
        gl,
        src_video_texture,
    }: DataForRendering,
) {
    renderer_data.use_program(&ProgramId::Unfiltered);
    renderer_data.use_vao(&VAOId::Quad);
    gl.active_texture(WebGl2RenderingContext::TEXTURE0 + TextureId::SrcVideo.location());
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(src_video_texture));
    gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
    draw(gl, canvas);
}

/// Renders using the Grayscale filter
pub(crate) fn render_grayscale(
    DataForRendering {
        canvas,
        renderer_data,
        gl,
        src_video_texture,
    }: DataForRendering,
) {
    renderer_data.use_program(&ProgramId::Grayscale);
    renderer_data.use_vao(&VAOId::Quad);
    gl.active_texture(WebGl2RenderingContext::TEXTURE0 + TextureId::SrcVideo.location());
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(src_video_texture));
    gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
    draw(gl, canvas);
}

/// Renders using the Invert filter
pub(crate) fn render_inverted(
    DataForRendering {
        canvas,
        renderer_data,
        gl,
        src_video_texture,
    }: DataForRendering,
) {
    renderer_data.use_program(&ProgramId::Invert);
    renderer_data.use_vao(&VAOId::Quad);
    gl.active_texture(WebGl2RenderingContext::TEXTURE0 + TextureId::SrcVideo.location());
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(src_video_texture));
    gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
    draw(gl, canvas);
}

/// Renders using the Wavy filter
pub(crate) fn render_wavy(
    DataForRendering {
        canvas,
        renderer_data,
        gl,
        src_video_texture,
    }: DataForRendering,
) {
    renderer_data.use_program(&ProgramId::Wavy);
    renderer_data.use_vao(&VAOId::Quad);
    gl.active_texture(WebGl2RenderingContext::TEXTURE0 + TextureId::SrcVideo.location());
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(src_video_texture));
    gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
    draw(gl, canvas);
}

/// Renders using the GaussianBlur filter
pub(crate) fn render_gaussian_blur(
    DataForRendering {
        canvas,
        renderer_data,
        gl,
        src_video_texture,
    }: DataForRendering,
) {
    renderer_data.use_program(&ProgramId::GaussianBlur);
    renderer_data.use_vao(&VAOId::Quad);
    gl.active_texture(WebGl2RenderingContext::TEXTURE0 + TextureId::SrcVideo.location());
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(src_video_texture));
    gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
    draw(gl, canvas);
}

pub(crate) struct DataForUploadingSrcVideoTexture<'a> {
    src_video_width: u32,
    src_video_height: u32,
    gl: &'a WebGl2RenderingContext,
    src_video_element: HtmlVideoElement,
    src_video_texture: &'a WebGlTexture,
}

/// If video has data to show (i.e. it is not zero width and height), data is uploaded as
/// as a texture for access from the fragment shaders, else uploads a single black pixel
/// as a texture to prevent stale data from being sampled in the fragment shaders
pub(crate) fn upload_src_video_as_texture(
    DataForUploadingSrcVideoTexture {
        src_video_width,
        src_video_height,
        gl,
        src_video_element,
        src_video_texture,
    }: DataForUploadingSrcVideoTexture,
) {
    if src_video_width > 0 && src_video_height > 0 {
        // upload video data as texture
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
    } else {
        // upload black pixel as texture to prevent reading from stale data
        gl.active_texture(WebGl2RenderingContext::TEXTURE0 + TextureId::SrcVideo.location());
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(src_video_texture));
        if let Err(err) = gl
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA as i32,
                1,
                1,
                0,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                None,
            )
        {
            error!(
                "Error uploading single-pixel blank WebGL texture: {:?}",
                err
            );
        }
    }
}

/// Adjusts dimensions of the canvas to match the dimensions of the source video
pub fn adjust_canvas_dimensions_to_match_video(
    canvas: &HtmlCanvasElement,
    src_video_width: u32,
    src_video_height: u32,
) {
    if canvas.width() != src_video_width {
        canvas.set_width(src_video_width)
    }
    if canvas.height() != src_video_height {
        canvas.set_height(src_video_height)
    }
}

/// Chooses the correct filter to render based on what is currently selected
pub(crate) fn render_any_filter<'a>(
    render_state_handle: &'a RenderStateHandle,
    data_for_rendering: DataForRendering,
) {
    match render_state_handle.borrow().filter_type() {
        FilterType::Unfiltered => render_unfiltered(data_for_rendering),
        FilterType::Grayscale => render_grayscale(data_for_rendering),
        FilterType::Invert => render_inverted(data_for_rendering),
        FilterType::Wavy => render_wavy(data_for_rendering),
        FilterType::GaussianBlur => render_gaussian_blur(data_for_rendering),
    }
}

pub fn render(
    renderer_data: &RendererData<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        String,
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

    adjust_canvas_dimensions_to_match_video(canvas, src_video_width, src_video_height);

    upload_src_video_as_texture(DataForUploadingSrcVideoTexture {
        src_video_width,
        src_video_height,
        gl,
        src_video_element,
        src_video_texture,
    });

    render_any_filter(
        render_state_handle,
        DataForRendering {
            renderer_data,
            gl,
            canvas,
            src_video_texture,
        },
    )
}
