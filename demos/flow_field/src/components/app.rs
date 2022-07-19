use crate::{
    graphics::{
        attribute_id::AttributeId,
        buffer_id::BufferId,
        create_buffer::{
            create_particle_buffer_a, create_particle_buffer_b, create_quad_vertex_buffer,
        },
        create_framebuffer::create_perlin_noise_framebuffer,
        create_position_attribute::{
            create_particle_position_attribute, create_quad_vertex_attribute,
        },
        create_texture::{create_perlin_noise_texture, create_white_noise_texture},
        fragment_shader_id::FragmentShaderId,
        framebuffer_id::FramebufferId,
        program_id::ProgramId,
        render::render,
        texture_id::TextureId,
        transform_feedback_id::TransformFeedbackId,
        uniform_id::UniformId,
        vertex_shader_id::VertexShaderId,
    },
    state::{render_state::RenderState, render_state_handle::RenderStateHandle},
};
use std::rc::Rc;
use ui::route::Route;
use web_sys::HtmlCanvasElement;
use wrend::{
    AnimationCallback, AttributeCreateCallback, AttributeLink, BufferCreateCallback, BufferLink,
    FramebufferCreateCallback, FramebufferLink, ProgramLinkBuilder, RenderCallback, Renderer,
    TextureCreateCallback, TextureLink, TransformFeedbackLink, UniformCallback, UniformContext,
    UniformLink,
};

use yew::{function_component, html, use_effect_with_deps, use_mut_ref, use_node_ref};
use yew_router::prelude::*;

const QUAD_VERTEX_SHADER: &str = include_str!("../shaders/quad.vert");
const PASS_THROUGH_FRAGMENT_SHADER: &str = include_str!("../shaders/pass_through.frag");
const PERLIN_NOISE_FRAGMENT_SHADER: &str = include_str!("../shaders/perlin_noise.frag");
const UPDATE_PARTICLES_FRAGMENT_SHADER: &str = include_str!("../shaders/update_particles.frag");
const UPDATE_PARTICLES_VERTEX_SHADER: &str = include_str!("../shaders/update_particles.vert");
const DRAW_PARTICLES_FRAGMENT_SHADER: &str = include_str!("../shaders/draw_particles.frag");
const DRAW_PARTICLES_VERTEX_SHADER: &str = include_str!("../shaders/draw_particles.vert");

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
                    .set_program_id(ProgramId::PassThrough)
                    .set_vertex_shader_id(VertexShaderId::Quad)
                    .set_fragment_shader_id(FragmentShaderId::PassThrough)
                    .build()
                    .expect("Should build PassThrough ProgramLink successfully");

                let perlin_noise_program_link = ProgramLinkBuilder::new()
                    .set_program_id(ProgramId::PerlinNoise)
                    .set_vertex_shader_id(VertexShaderId::Quad)
                    .set_fragment_shader_id(FragmentShaderId::PerlinNoise)
                    .build()
                    .expect("Should build PerlinNoise ProgramLink successfully");

                let update_particles_program_link = ProgramLinkBuilder::new()
                    .set_program_id(ProgramId::UpdateParticles)
                    .set_vertex_shader_id(VertexShaderId::UpdateParticles)
                    .set_fragment_shader_id(FragmentShaderId::UpdateParticles)
                    .set_transform_feedback_varyings(["o_position".to_string()])
                    .build()
                    .expect("Should build UpdateParticles ProgramLink successfully");

                let draw_particles_program_link = ProgramLinkBuilder::new()
                    .set_program_id(ProgramId::DrawParticles)
                    .set_vertex_shader_id(VertexShaderId::DrawParticles)
                    .set_fragment_shader_id(FragmentShaderId::DrawParticles)
                    .build()
                    .expect("Should build DrawParticles ProgramLink successfully");

                let vertex_buffer_link = BufferLink::new(
                    BufferId::QuadVertexBuffer,
                    BufferCreateCallback::new(Rc::new(create_quad_vertex_buffer)),
                );

                let particle_buffer_a_link = BufferLink::new(
                    BufferId::ParticleBufferA,
                    BufferCreateCallback::new(Rc::new(create_particle_buffer_a)),
                );

                let particle_buffer_b_link = BufferLink::new(
                    BufferId::ParticleBufferB,
                    BufferCreateCallback::new(Rc::new(create_particle_buffer_b)),
                );

                let a_particle_position_a_link = AttributeLink::new(
                    (ProgramId::UpdateParticles, ProgramId::DrawParticles),
                    BufferId::ParticleBufferA,
                    AttributeId::AParticlePositionA,
                    AttributeCreateCallback::new(Rc::new(create_particle_position_attribute)),
                );

                let a_particle_position_b_link = AttributeLink::new(
                    (ProgramId::UpdateParticles, ProgramId::DrawParticles),
                    BufferId::ParticleBufferB,
                    AttributeId::AParticlePositionB,
                    AttributeCreateCallback::new(Rc::new(create_particle_position_attribute)),
                );

                let a_quad_vertex_link = AttributeLink::new(
                    (ProgramId::PassThrough, ProgramId::PerlinNoise),
                    BufferId::QuadVertexBuffer,
                    AttributeId::AQuadVertex,
                    AttributeCreateCallback::new(Rc::new(create_quad_vertex_attribute)),
                );

                let white_noise_texture_link = TextureLink::new(
                    TextureId::WhiteNoise,
                    TextureCreateCallback::new(Rc::new(create_white_noise_texture)),
                );

                let u_white_noise_texture = UniformLink::new(
                    ProgramId::PerlinNoise,
                    UniformId::UWhiteNoiseTexture,
                    UniformCallback::new(Rc::new(|ctx| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1i(Some(uniform_location), 0);
                    })),
                );

                let perlin_noise_texture_link = TextureLink::new(
                    TextureId::PerlinNoise,
                    TextureCreateCallback::new(Rc::new(create_perlin_noise_texture)),
                );

                let u_perlin_noise_texture = UniformLink::new(
                    (ProgramId::PassThrough, ProgramId::UpdateParticles, ProgramId::DrawParticles),
                    UniformId::UPerlinNoiseTexture,
                    UniformCallback::new(Rc::new(|ctx| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1i(Some(uniform_location), 1);
                    })),
                );

                let perlin_noise_framebuffer_link = FramebufferLink::new(
                    FramebufferId::PerlinNoise,
                    FramebufferCreateCallback::new(Rc::new(create_perlin_noise_framebuffer)),
                    Some(TextureId::PerlinNoise),
                );

                let u_now_link_init_and_update_callback =
                    Rc::new(|ctx: &UniformContext<RenderStateHandle>| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1f(Some(uniform_location), (ctx.now() / 2000.) as f32);
                    });

                let mut u_now = UniformLink::new(
                    ProgramId::PerlinNoise,
                    UniformId::UNow,
                    UniformCallback::new(u_now_link_init_and_update_callback.clone()),
                );

                u_now.set_update_callback(UniformCallback::new(
                    u_now_link_init_and_update_callback.clone(),
                ));

                let transform_feedback_link =
                    TransformFeedbackLink::new(TransformFeedbackId::Particle);

                let render_callback = RenderCallback::new(Rc::new(render));
                let render_state_handle: RenderStateHandle = render_state.into();

                let mut renderer_builder = Renderer::builder();
                renderer_builder
                    .set_canvas(canvas)
                    .set_user_ctx(render_state_handle)
                    .set_render_callback(render_callback)
                    .add_vertex_shader_src(VertexShaderId::Quad, QUAD_VERTEX_SHADER.to_string())
                    .add_fragment_shader_src(
                        FragmentShaderId::PerlinNoise,
                        PERLIN_NOISE_FRAGMENT_SHADER.to_string(),
                    )
                    .add_fragment_shader_src(
                        FragmentShaderId::PassThrough,
                        PASS_THROUGH_FRAGMENT_SHADER.to_string(),
                    )
                    .add_fragment_shader_src(
                        FragmentShaderId::UpdateParticles,
                        UPDATE_PARTICLES_FRAGMENT_SHADER.to_string(),
                    )
                    .add_vertex_shader_src(
                        VertexShaderId::UpdateParticles,
                        UPDATE_PARTICLES_VERTEX_SHADER.to_string(),
                    )
                    .add_fragment_shader_src(
                        FragmentShaderId::DrawParticles,
                        DRAW_PARTICLES_FRAGMENT_SHADER.to_string(),
                    )
                    .add_vertex_shader_src(
                        VertexShaderId::DrawParticles,
                        DRAW_PARTICLES_VERTEX_SHADER.to_string(),
                    )
                    .add_program_link(perlin_noise_program_link)
                    .add_program_link(pass_through_program_link)
                    .add_program_link(update_particles_program_link)
                    .add_program_link(draw_particles_program_link)
                    .add_buffer_link(vertex_buffer_link)
                    .add_buffer_link(particle_buffer_a_link)
                    .add_buffer_link(particle_buffer_b_link)
                    .add_attribute_link(a_quad_vertex_link)
                    .add_attribute_link(a_particle_position_a_link)
                    .add_attribute_link(a_particle_position_b_link)
                    .add_texture_link(perlin_noise_texture_link)
                    .add_texture_link(white_noise_texture_link)
                    .add_framebuffer_link(perlin_noise_framebuffer_link)
                    .add_uniform_link(u_perlin_noise_texture)
                    .add_uniform_link(u_white_noise_texture)
                    .add_uniform_link(u_now)
                    .add_transform_feedback_link(transform_feedback_link);

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
        <div class="flow-field">
            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            <canvas ref={canvas_ref} height={1000} width={1000} />
        </div>
    }
}
