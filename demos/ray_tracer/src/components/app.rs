use crate::{
    graphics::{
        attribute_id::AttributeId,
        buffer_id::BufferId,
        create_buffer::create_quad_vertex_buffer,
        create_framebuffer::create_render_framebuffer,
        create_position_attribute::create_position_attribute,
        create_texture::make_create_render_texture,
        create_uniforms::{
            create_general_ray_tracer_uniform_links, create_general_uniform_links,
            create_sphere_uniform_links,
        },
        fragment_shader_id::FragmentShaderId,
        framebuffer_id::FramebufferId,
        program_id::ProgramId,
        render::render,
        texture_id::TextureId,
        vao_id::VAOId,
        vertex_shader_id::VertexShaderId,
    },
    state::{app_state::AppState, state_handle::StateHandle},
    utils::clamped_screen_dimensions,
};
use std::rc::Rc;
use ui::route::Route;
use web_sys::HtmlCanvasElement;
use wrend::{
    AnimationCallback, AttributeCreateCallback, AttributeLink, BufferCreateCallback, BufferLink,
    FramebufferCreateCallback, FramebufferLink, ProgramLinkBuilder, RenderCallback, Renderer,
    TextureCreateCallback, TextureLink,
};

use yew::{function_component, html, use_effect_with_deps, use_mut_ref, use_node_ref};
use yew_router::prelude::*;

const QUAD_VERTEX_SHADER: &str = include_str!("../shaders/quad_vertex.glsl");
const RAY_TRACER_FRAGMENT_SHADER: &str = include_str!("../shaders/ray_tracer.glsl");
const AVERAGE_RENDERS_FRAGMENT_SHADERS: &str = include_str!("../shaders/average_renders.glsl");
const PASS_THROUGH_FRAGMENT_SHADER: &str = include_str!("../shaders/pass_through.glsl");

#[function_component(App)]
pub fn app() -> Html {
    let canvas_ref = use_node_ref();
    let app_state = use_mut_ref(AppState::default);
    let animation_handle = use_mut_ref(|| None);

    use_effect_with_deps(
        {
            let canvas_ref = canvas_ref.clone();
            let animation_handle = animation_handle;
            move |_| {
                let canvas: HtmlCanvasElement = canvas_ref
                    .cast()
                    .expect("Canvas ref should point to a canvas in the use_effect hook");

                let ray_tracer_program_link = ProgramLinkBuilder::new()
                    .set_vertex_shader_id(VertexShaderId::Quad)
                    .set_program_id(ProgramId::RayTracer)
                    .set_fragment_shader_id(FragmentShaderId::RayTracer)
                    .build()
                    .expect("Should build RayTracer ProgramLink successfully");

                let average_renders_program_link = ProgramLinkBuilder::new()
                    .set_vertex_shader_id(VertexShaderId::Quad)
                    .set_program_id(ProgramId::AverageRenders)
                    .set_fragment_shader_id(FragmentShaderId::AverageRenders)
                    .build()
                    .expect("Should build AverageRenders ProgramLink successfully");

                let pass_through_program_link = ProgramLinkBuilder::new()
                    .set_vertex_shader_id(VertexShaderId::Quad)
                    .set_program_id(ProgramId::PassThrough)
                    .set_fragment_shader_id(FragmentShaderId::PassThrough)
                    .build()
                    .expect("Should build PassThrough ProgramLink successfully");

                let vertex_buffer_link = BufferLink::new(
                    BufferId::QuadVertexBuffer,
                    BufferCreateCallback::new(Rc::new(create_quad_vertex_buffer)),
                );

                let a_quad_vertex_link = AttributeLink::new(
                    VAOId::Quad,
                    BufferId::QuadVertexBuffer,
                    AttributeId,
                    AttributeCreateCallback::new(Rc::new(create_position_attribute)),
                );

                let prev_render_texture_link = TextureLink::new(
                    TextureId::PrevRender,
                    TextureCreateCallback::new(make_create_render_texture(TextureId::PrevRender)),
                );

                let averaged_render_a_texture_link = TextureLink::new(
                    TextureId::AveragedRenderA,
                    TextureCreateCallback::new(make_create_render_texture(
                        TextureId::AveragedRenderA,
                    )),
                );

                let averaged_render_b_texture_link = TextureLink::new(
                    TextureId::AveragedRenderB,
                    TextureCreateCallback::new(make_create_render_texture(
                        TextureId::AveragedRenderB,
                    )),
                );

                let prev_render_framebuffer_link = FramebufferLink::new(
                    FramebufferId::PrevRender,
                    FramebufferCreateCallback::new(Rc::new(create_render_framebuffer)),
                    Some(TextureId::PrevRender),
                );

                let averaged_render_a_framebuffer_link = FramebufferLink::new(
                    FramebufferId::AveragedRenderA,
                    FramebufferCreateCallback::new(Rc::new(create_render_framebuffer)),
                    Some(TextureId::AveragedRenderA),
                );

                let averaged_render_b_framebuffer_link = FramebufferLink::new(
                    FramebufferId::AveragedRenderB,
                    FramebufferCreateCallback::new(Rc::new(create_render_framebuffer)),
                    Some(TextureId::AveragedRenderB),
                );

                let render_callback = RenderCallback::new(Rc::new(render));

                let state_handle = StateHandle::new(app_state);

                // sync state and canvas size
                {
                    let (width, height) = clamped_screen_dimensions();
                    let mut app_state_ref = state_handle.borrow_mut();
                    let render_state_mut = app_state_ref.render_state_mut();
                    render_state_mut.width = width;
                    render_state_mut.height = height;
                    render_state_mut.update_pipeline();
                    canvas.set_width(render_state_mut.width);
                    canvas.set_height(render_state_mut.height);
                }

                let mut renderer_builder = Renderer::builder();

                renderer_builder
                    .set_canvas(canvas)
                    .set_user_ctx(state_handle)
                    .set_render_callback(render_callback)
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
                    .add_uniform_links(create_general_uniform_links())
                    .add_uniform_links(create_general_ray_tracer_uniform_links())
                    .add_uniform_links(create_sphere_uniform_links())
                    .add_vao_link(VAOId::Quad);

                let renderer = renderer_builder
                    .build()
                    .expect("Renderer should successfully build");

                renderer.update_uniforms();
                renderer.render();
                let new_animation_handle =
                    renderer.into_animation_handle(AnimationCallback::new(Rc::new(|renderer| {
                        renderer.update_uniforms();
                        renderer.render();
                    })));

                new_animation_handle.start_animating();

                // save handle to keep animation going
                *animation_handle.borrow_mut() = Some(new_animation_handle);

                || {}
            }
        },
        (),
    );

    html! {
        <div class="ray-tracer">
            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            <canvas ref={canvas_ref} />
        </div>
    }
}
