use crate::{
    graphics::{
        attribute_id::AttributeId, buffer_id::BufferId, create_buffer::create_quad_vertex_buffer,
        create_framebuffer::create_render_framebuffer,
        create_position_attribute::create_position_attribute,
        create_texture::create_ray_tracer_texture, fragment_shader_id::FragmentShaderId,
        framebuffer_id::FramebufferId, program_id::ProgramId, render::render,
        texture_id::TextureId, uniform_id::UniformId, vao_id::VAOId,
        vertex_shader_id::VertexShaderId,
    },
    state::{app_state::AppState, state_handle::StateHandle},
};
use std::rc::Rc;
use ui::route::Route;
use web_sys::HtmlCanvasElement;
use wrend::{
    AnimationCallback, AttributeCreateCallback, AttributeLink, BufferCreateCallback, BufferLink,
    FramebufferCreateCallback, FramebufferLink, ProgramLinkBuilder, RenderCallback, Renderer,
    TextureCreateCallback, TextureLink, UniformCallback, UniformContext, UniformLink,
};

use yew::{function_component, html, use_effect_with_deps, use_mut_ref, use_node_ref};
use yew_router::prelude::*;

const VERTEX_SHADER: &str = include_str!("../shaders/vertex.glsl");
const PASS_THROUGH_FRAGMENT_SHADER: &str = include_str!("../shaders/pass_through.glsl");
const PERLIN_NOISE_FRAGMENT_SHADER: &str = include_str!("../shaders/ray_tracer.glsl");

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

                let pass_through_program_link = ProgramLinkBuilder::new()
                    .set_vertex_shader_id(VertexShaderId::Quad)
                    .set_program_id(ProgramId::PassThrough)
                    .set_fragment_shader_id(FragmentShaderId::PassThrough)
                    .build()
                    .expect("Should build PassThrough ProgramLink successfully");

                let ray_tracer_program_link = ProgramLinkBuilder::new()
                    .set_vertex_shader_id(VertexShaderId::Quad)
                    .set_program_id(ProgramId::RayTracer)
                    .set_fragment_shader_id(FragmentShaderId::RayTracer)
                    .build()
                    .expect("Should build RayTracer ProgramLink successfully");

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

                let render_a_texture_link = TextureLink::new(
                    TextureId::RenderA,
                    TextureCreateCallback::new(Rc::new(create_ray_tracer_texture)),
                );

                let render_b_texture_link = TextureLink::new(
                    TextureId::RenderB,
                    TextureCreateCallback::new(Rc::new(create_ray_tracer_texture)),
                );

                let u_render_a_texture = UniformLink::new(
                    ProgramId::PassThrough,
                    UniformId::URenderATexture,
                    UniformCallback::new(Rc::new(|ctx| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1ui(Some(uniform_location), TextureId::RenderA.location());
                    })),
                );

                let u_render_b_texture = UniformLink::new(
                    ProgramId::PassThrough,
                    UniformId::URenderBTexture,
                    UniformCallback::new(Rc::new(|ctx| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1ui(Some(uniform_location), TextureId::RenderB.location());
                    })),
                );

                let render_a_framebuffer_link = FramebufferLink::new(
                    FramebufferId::RenderA,
                    FramebufferCreateCallback::new(Rc::new(create_render_framebuffer)),
                    Some(TextureId::RenderA),
                );

                let render_b_framebuffer_link = FramebufferLink::new(
                    FramebufferId::RenderB,
                    FramebufferCreateCallback::new(Rc::new(create_render_framebuffer)),
                    Some(TextureId::RenderB),
                );

                let u_now_link_init_and_update_callback =
                    Rc::new(|ctx: &UniformContext<StateHandle>| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1f(Some(uniform_location), (ctx.now() / 2000.) as f32);
                    });

                let mut u_now = UniformLink::new(
                    ProgramId::RayTracer,
                    UniformId::UNow,
                    UniformCallback::new(u_now_link_init_and_update_callback.clone()),
                );

                u_now.set_update_callback(UniformCallback::new(
                    u_now_link_init_and_update_callback.clone(),
                ));

                let render_callback = RenderCallback::new(Rc::new(render));

                let app_state_handle = StateHandle::new(app_state);

                let mut renderer_builder = Renderer::builder();

                renderer_builder
                    .set_canvas(canvas)
                    .set_user_ctx(app_state_handle)
                    .set_render_callback(render_callback)
                    .add_vertex_shader_src(VertexShaderId::Quad, VERTEX_SHADER.to_string())
                    .add_fragment_shader_src(
                        FragmentShaderId::RayTracer,
                        PERLIN_NOISE_FRAGMENT_SHADER.to_string(),
                    )
                    .add_fragment_shader_src(
                        FragmentShaderId::PassThrough,
                        PASS_THROUGH_FRAGMENT_SHADER.to_string(),
                    )
                    .add_program_link(pass_through_program_link)
                    .add_program_link(ray_tracer_program_link)
                    .add_buffer_link(vertex_buffer_link)
                    .add_attribute_link(a_quad_vertex_link)
                    .add_texture_link(render_a_texture_link)
                    .add_texture_link(render_b_texture_link)
                    .add_framebuffer_link(render_a_framebuffer_link)
                    .add_framebuffer_link(render_b_framebuffer_link)
                    .add_uniform_link(u_now)
                    .add_uniform_link(u_render_a_texture)
                    .add_uniform_link(u_render_b_texture)
                    .add_vao_link(VAOId::Quad);

                let renderer = renderer_builder
                    .build()
                    .expect("Renderer should successfully build");

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
            <canvas ref={canvas_ref} height={1000} width={1000} />
        </div>
    }
}
