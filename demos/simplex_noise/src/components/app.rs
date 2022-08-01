use crate::{
    graphics::{
        attribute_id::AttributeId, buffer_id::BufferId, create_buffer::create_vertex_buffer,
        create_framebuffer::create_frame_buffer,
        create_position_attribute::create_position_attribute,
        create_texture::create_simplex_noise_texture, fragment_shader_id::FragmentShaderId,
        framebuffer_id::FramebufferId, program_id::ProgramId, render::render,
        texture_id::TextureId, uniform_id::UniformId, vertex_shader_id::VertexShaderId,
    },
    state::{render_state::RenderState, render_state_handle::RenderStateHandle},
};
use std::rc::Rc;
use ui::route::Route;
use web_sys::HtmlCanvasElement;
use wrend::{
    AnimationCallback, AttributeCreateCallback, AttributeLink, BufferCreateCallback, BufferLink,
    FramebufferCreateCallback, FramebufferLink, ProgramLinkBuilder, RenderCallback, Renderer,
    TextureCreateCallback, TextureLink, UniformContext, UniformLink,
};

use yew::{function_component, html, use_effect_with_deps, use_mut_ref, use_node_ref};
use yew_router::prelude::*;

const VERTEX_SHADER: &str = include_str!("../shaders/vertex.glsl");
const PASS_THROUGH_FRAGMENT_SHADER: &str = include_str!("../shaders/pass_through.glsl");
const PERLIN_NOISE_FRAGMENT_SHADER: &str = include_str!("../shaders/simplex_noise.glsl");

#[function_component(App)]
pub fn app() -> Html {
    let canvas_ref = use_node_ref();
    let render_state = use_mut_ref(RenderState::default);
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

                let simplex_noise_program_link = ProgramLinkBuilder::new()
                    .set_vertex_shader_id(VertexShaderId::Quad)
                    .set_program_id(ProgramId::SimplexNoise)
                    .set_fragment_shader_id(FragmentShaderId::SimplexNoise)
                    .build()
                    .expect("Should build SimplexNoise ProgramLink successfully");

                let vertex_buffer_link = BufferLink::new(
                    BufferId::VertexBuffer,
                    BufferCreateCallback::new(Rc::new(create_vertex_buffer)),
                );

                let a_position_link = AttributeLink::new(
                    (ProgramId::PassThrough, ProgramId::SimplexNoise),
                    BufferId::VertexBuffer,
                    AttributeId,
                    AttributeCreateCallback::new(Rc::new(create_position_attribute)),
                );

                let simplex_noise_texture_link = TextureLink::new(
                    TextureId::SimplexNoise,
                    TextureCreateCallback::new(Rc::new(create_simplex_noise_texture)),
                );

                let u_simplex_noise_texture = UniformLink::new(
                    ProgramId::PassThrough,
                    UniformId::USimplexNoiseTexture,
                    |ctx: &UniformContext<_>| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1i(Some(uniform_location), 1);
                    },
                );

                let simplex_noise_framebuffer_link = FramebufferLink::new(
                    FramebufferId::SimplexNoise,
                    FramebufferCreateCallback::new(Rc::new(create_frame_buffer)),
                    Some(TextureId::SimplexNoise),
                );

                let u_now_link_init_and_update_callback =
                    Rc::new(|ctx: &UniformContext<RenderStateHandle>| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1f(Some(uniform_location), (ctx.now() / 2000.) as f32);
                    });

                let mut u_now = UniformLink::new(
                    ProgramId::SimplexNoise,
                    UniformId::UNow,
                    u_now_link_init_and_update_callback.clone(),
                );

                u_now.set_update_callback(u_now_link_init_and_update_callback.clone());

                let render_callback = RenderCallback::new(Rc::new(render));
                let render_state_handle: RenderStateHandle = render_state.into();

                let mut renderer_builder = Renderer::builder();

                renderer_builder
                    .set_canvas(canvas)
                    .set_user_ctx(render_state_handle)
                    .set_render_callback(render_callback)
                    .add_vertex_shader_src(VertexShaderId::Quad, VERTEX_SHADER.to_string())
                    .add_fragment_shader_src(
                        FragmentShaderId::SimplexNoise,
                        PERLIN_NOISE_FRAGMENT_SHADER.to_string(),
                    )
                    .add_fragment_shader_src(
                        FragmentShaderId::PassThrough,
                        PASS_THROUGH_FRAGMENT_SHADER.to_string(),
                    )
                    .add_program_link(pass_through_program_link)
                    .add_program_link(simplex_noise_program_link)
                    .add_buffer_link(vertex_buffer_link)
                    .add_attribute_link(a_position_link)
                    .add_uniform_link(u_now)
                    .add_uniform_link(u_simplex_noise_texture)
                    .add_texture_link(simplex_noise_texture_link)
                    .add_framebuffer_link(simplex_noise_framebuffer_link)
                    .add_vao_link(ProgramId::PassThrough)
                    .add_vao_link(ProgramId::SimplexNoise);

                let renderer = renderer_builder
                    .build()
                    .expect("Renderer should successfully build");

                let new_animation_handle = renderer.into_animation_handle(AnimationCallback::new(
                    Rc::new(|renderer: &Renderer<_, _, _, _, _, _, _, _, _, _, _>| {
                        renderer.update_uniforms();
                        renderer.render();
                    }),
                ));

                new_animation_handle.start_animating();

                // save handle to keep animation going
                *animation_handle.borrow_mut() = Some(new_animation_handle);

                || {}
            }
        },
        (),
    );

    html! {
        <div class="simplex-noise">
            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            <canvas ref={canvas_ref} height={1000} width={1000} />
        </div>
    }
}
