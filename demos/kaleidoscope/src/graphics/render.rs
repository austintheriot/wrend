use super::{
    attribute_id::AttributeId, buffer_id::BufferId, fragment_shader_id::FragmentShaderId,
    framebuffer_id::FramebufferId, program_id::ProgramId, texture_id::TextureId,
    transform_feedback_id::TransformFeedbackId, vertex_shader_id::VertexShaderId, FilterType,
    GenerationType, VAOId,
};
use crate::state::AppStateHandle;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlFramebuffer, WebGlTexture};
use wrend::RendererData;

/// Reusable draw call for multiple filter types
fn draw(gl: &WebGl2RenderingContext, canvas: &HtmlCanvasElement) {
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
    gl.clear_color(0.0, 0.0, 0.0, 0.0);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);
}

pub struct DataForRendering<'a> {
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
        AppStateHandle,
    >,
    gl: &'a WebGl2RenderingContext,
    canvas: &'a HtmlCanvasElement,
    src_texture: &'a WebGlTexture,
    dest_framebuffer: Option<&'a WebGlFramebuffer>,
}

/// Generates a src texture using the Circle Gradient fragment shader
pub fn generate_circle_gradient(
    DataForRendering {
        canvas,
        renderer_data,
        gl,
        dest_framebuffer,
        ..
    }: &DataForRendering,
) {
    renderer_data.use_program(&ProgramId::GenerateCircleGradient);
    renderer_data.use_vao(&VAOId::Quad);
    gl.bind_framebuffer(
        WebGl2RenderingContext::FRAMEBUFFER,
        dest_framebuffer.as_deref(),
    );
    draw(gl, canvas);
}

/// Chooses the correct generation shader to generate the src texture
pub fn generate_src_texture<'a>(
    app_state_handle: &'a AppStateHandle,
    data_for_generating: &DataForRendering,
) {
    match *app_state_handle
        .borrow()
        .ui_state()
        .generation_type_ref()
        .borrow()
    {
        GenerationType::CircleGradient => generate_circle_gradient(data_for_generating),
    }
}

/// Renders using the Unfiltered filter
pub fn render_filter_unfiltered(
    DataForRendering {
        canvas,
        renderer_data,
        gl,
        src_texture,
        dest_framebuffer,
    }: &DataForRendering,
) {
    renderer_data.use_program(&ProgramId::FilterUnfiltered);
    renderer_data.use_vao(&VAOId::Quad);
    gl.active_texture(WebGl2RenderingContext::TEXTURE0 + TextureId::SrcTexture.location());
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(src_texture));
    gl.bind_framebuffer(
        WebGl2RenderingContext::FRAMEBUFFER,
        dest_framebuffer.as_deref(),
    );
    draw(gl, canvas);
}

/// Renders using the Split filter
pub fn render_filter_split(
    DataForRendering {
        canvas,
        renderer_data,
        gl,
        src_texture,
        dest_framebuffer,
    }: &DataForRendering,
) {
    renderer_data.use_program(&ProgramId::FilterSplit);
    renderer_data.use_vao(&VAOId::Quad);
    gl.active_texture(WebGl2RenderingContext::TEXTURE0 + TextureId::SrcTexture.location());
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(src_texture));
    gl.bind_framebuffer(
        WebGl2RenderingContext::FRAMEBUFFER,
        dest_framebuffer.as_deref(),
    );
    draw(gl, canvas);
}

/// Chooses the correct filter to render based on what is currently selected
pub fn render_filter<'a>(
    app_state_handle: &'a AppStateHandle,
    data_for_filtering: &DataForRendering,
) {
    match (*app_state_handle.borrow().as_ref().applied_filters())
        .first()
        .map(Clone::clone)
        .unwrap_or_default()
    {
        FilterType::Unfiltered => render_filter_unfiltered(data_for_filtering),
        FilterType::Split => render_filter_split(data_for_filtering),
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
        AppStateHandle,
    >,
) {
    let gl = renderer_data.gl();
    let canvas = renderer_data.canvas();
    let app_state_handle = renderer_data.user_ctx().unwrap();
    let src_texture = renderer_data
        .texture(&TextureId::SrcTexture)
        .unwrap()
        .webgl_texture();
    let src_texture_framebuffer = renderer_data
        .framebuffer(&FramebufferId::SrcTexture)
        .unwrap()
        .webgl_framebuffer();

    // render into a framebuffer
    generate_src_texture(
        app_state_handle,
        &DataForRendering {
            renderer_data,
            gl,
            canvas,
            src_texture,
            dest_framebuffer: Some(src_texture_framebuffer),
        },
    );

    // for now, test rendering directly to the canvas
    render_filter(
        app_state_handle,
        &DataForRendering {
            renderer_data,
            gl,
            canvas,
            src_texture,
            dest_framebuffer: None,
        },
    );
}
