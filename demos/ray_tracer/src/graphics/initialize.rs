use crate::{
    graphics::{
        animate::animate,
        attribute_id::AttributeId,
        buffer_id::BufferId,
        create_buffer::create_quad_vertex_buffer,
        create_framebuffer::create_render_framebuffer,
        create_position_attribute::create_position_attribute,
        create_texture::make_create_render_texture,
        create_uniforms::{
            create_general_ray_tracer_uniform_links, create_shared_uniform_links,
            create_sphere_uniform_links,
        },
        fragment_shader_id::FragmentShaderId,
        framebuffer_id::FramebufferId,
        program_id::ProgramId,
        render::render,
        texture_id::TextureId,
        transform_feedback_id::TransformFeedbackId,
        vao_id::VAOId,
        vertex_shader_id::VertexShaderId,
    },
    state::app_context::AppContext,
};

use web_sys::HtmlCanvasElement;
use wrend::{
    AttributeLink, BufferLink, FramebufferLink, ProgramLinkBuilder, Renderer, RendererHandle,
    TextureLink,
};

const QUAD_VERTEX_SHADER: &str = include_str!("../shaders/quad_vertex.glsl");
const RAY_TRACER_FRAGMENT_SHADER: &str = include_str!("../shaders/ray_tracer.glsl");
const AVERAGE_RENDERS_FRAGMENT_SHADERS: &str = include_str!("../shaders/average_renders.glsl");
const PASS_THROUGH_FRAGMENT_SHADER: &str = include_str!("../shaders/pass_through.glsl");

/// Builds the entire graphics pipeline / renderer and returns a handle to the started animation
pub fn build_renderer(
    canvas: HtmlCanvasElement,
    app_context: AppContext,
) -> RendererHandle<
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
    AppContext,
> {
    let mut ray_tracer_program_link = ProgramLinkBuilder::new();
    ray_tracer_program_link
        .set_vertex_shader_id(VertexShaderId::Quad)
        .set_program_id(ProgramId::RayTracer)
        .set_fragment_shader_id(FragmentShaderId::RayTracer);
    let ray_tracer_program_link = ray_tracer_program_link
        .build()
        .expect("Should build RayTracer ProgramLink successfully");

    let mut average_renders_program_link = ProgramLinkBuilder::new();
    average_renders_program_link
        .set_vertex_shader_id(VertexShaderId::Quad)
        .set_program_id(ProgramId::AverageRenders)
        .set_fragment_shader_id(FragmentShaderId::AverageRenders);
    let average_renders_program_link = average_renders_program_link
        .build()
        .expect("Should build AverageRenders ProgramLink successfully");

    let mut pass_through_program_link = ProgramLinkBuilder::new();
    pass_through_program_link
        .set_vertex_shader_id(VertexShaderId::Quad)
        .set_program_id(ProgramId::PassThrough)
        .set_fragment_shader_id(FragmentShaderId::PassThrough);
    let pass_through_program_link = pass_through_program_link
        .build()
        .expect("Should build PassThrough ProgramLink successfully");

    let vertex_buffer_link = BufferLink::new(BufferId::QuadVertexBuffer, create_quad_vertex_buffer);

    let a_quad_vertex_link = AttributeLink::new(
        VAOId::Quad,
        BufferId::QuadVertexBuffer,
        AttributeId,
        create_position_attribute,
    );

    let prev_render_texture_link = TextureLink::new(
        TextureId::PrevRender,
        make_create_render_texture(TextureId::PrevRender),
    );

    let averaged_render_a_texture_link = TextureLink::new(
        TextureId::AveragedRenderA,
        make_create_render_texture(TextureId::AveragedRenderA),
    );

    let averaged_render_b_texture_link = TextureLink::new(
        TextureId::AveragedRenderB,
        make_create_render_texture(TextureId::AveragedRenderB),
    );

    let prev_render_framebuffer_link = FramebufferLink::new(
        FramebufferId::PrevRender,
        create_render_framebuffer,
        Some(TextureId::PrevRender),
    );

    let averaged_render_a_framebuffer_link = FramebufferLink::new(
        FramebufferId::AveragedRenderA,
        create_render_framebuffer,
        Some(TextureId::AveragedRenderA),
    );

    let averaged_render_b_framebuffer_link = FramebufferLink::new(
        FramebufferId::AveragedRenderB,
        create_render_framebuffer,
        Some(TextureId::AveragedRenderB),
    );

    let mut renderer_builder = Renderer::builder();

    renderer_builder
        .set_canvas(canvas)
        .set_user_ctx(app_context.clone())
        .set_render_callback(render)
        .add_vertex_shader_src(VertexShaderId::Quad, QUAD_VERTEX_SHADER.to_string())
        .add_fragment_shader_src(
            FragmentShaderId::RayTracer,
            RAY_TRACER_FRAGMENT_SHADER.to_string(),
        )
        .add_fragment_shader_src(
            FragmentShaderId::PassThrough,
            PASS_THROUGH_FRAGMENT_SHADER.to_string(),
        )
        .add_fragment_shader_src(
            FragmentShaderId::AverageRenders,
            AVERAGE_RENDERS_FRAGMENT_SHADERS.to_string(),
        )
        .add_program_links([
            ray_tracer_program_link,
            average_renders_program_link,
            pass_through_program_link,
        ])
        .add_buffer_link(vertex_buffer_link)
        .add_attribute_link(a_quad_vertex_link)
        .add_texture_links([
            prev_render_texture_link,
            averaged_render_a_texture_link,
            averaged_render_b_texture_link,
        ])
        .add_framebuffer_links([
            prev_render_framebuffer_link,
            averaged_render_a_framebuffer_link,
            averaged_render_b_framebuffer_link,
        ])
        .add_uniform_links(create_general_ray_tracer_uniform_links(app_context.clone()))
        .add_uniform_links(create_sphere_uniform_links(app_context.clone()))
        .add_uniform_links(create_shared_uniform_links(app_context))
        .add_vao_link(VAOId::Quad);

    let renderer = renderer_builder
        .build()
        .expect("Renderer should successfully build");

    let mut new_renderer_handle = renderer.into_renderer_handle();
    new_renderer_handle.set_animation_callback(Some(animate));

    new_renderer_handle.start_animating();

    new_renderer_handle
}
