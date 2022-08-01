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
        vao_id::VAOId,
        vertex_shader_id::VertexShaderId,
    },
    state::{render_state::RenderState, render_state_handle::RenderStateHandle},
};
use std::rc::Rc;
use ui::route::Route;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, MouseEvent, WebGl2RenderingContext, WebGlContextAttributes};
use wrend::{
    AnimationCallback, AttributeCreateCallback, AttributeLink, BufferCreateCallback, BufferLink,
    CallbackWithContext, FramebufferCreateCallback, FramebufferLink, GetContextCallback,
    ProgramLinkBuilder, RenderCallback, Renderer, TextureCreateCallback, TextureLink,
    TransformFeedbackLink, UniformContext, UniformLink, WebGlContextError,
};

use yew::{function_component, html, use_effect_with_deps, use_mut_ref, use_node_ref, Callback};
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
            let render_state = Rc::clone(&render_state);
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

                let a_particle_position_link_a = AttributeLink::new(
                    (VAOId::DrawParticles, VAOId::UpdateParticlesA),
                    BufferId::ParticleBufferA,
                    AttributeId::AParticlePosition,
                    AttributeCreateCallback::new(Rc::new(create_particle_position_attribute)),
                );

                let a_particle_position_link_b = AttributeLink::new(
                    (VAOId::DrawParticles, VAOId::UpdateParticlesB),
                    BufferId::ParticleBufferB,
                    AttributeId::AParticlePosition,
                    AttributeCreateCallback::new(Rc::new(create_particle_position_attribute)),
                );

                let a_quad_vertex_link = AttributeLink::new(
                    (VAOId::PassThrough, VAOId::PerlinNoise),
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
                    |ctx: &UniformContext<_>| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1i(Some(uniform_location), 0);
                    },
                );

                let perlin_noise_texture_link = TextureLink::new(
                    TextureId::PerlinNoise,
                    TextureCreateCallback::new(Rc::new(create_perlin_noise_texture)),
                );

                let u_perlin_noise_texture = UniformLink::new(
                    (ProgramId::PassThrough, ProgramId::UpdateParticles),
                    UniformId::UPerlinNoiseTexture,
                    |ctx: &UniformContext<_>| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1i(Some(uniform_location), 1);
                    },
                );

                let perlin_noise_framebuffer_link = FramebufferLink::new(
                    FramebufferId::PerlinNoise,
                    FramebufferCreateCallback::new(create_perlin_noise_framebuffer),
                    Some(TextureId::PerlinNoise),
                );

                let u_now_link_init_and_update_callback =
                    Rc::new(|ctx: &UniformContext<RenderStateHandle>| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1f(Some(uniform_location), (ctx.now() / 50_000.) as f32);
                    });

                let u_now = UniformLink::new(
                    ProgramId::PerlinNoise,
                    UniformId::UNow,
                    u_now_link_init_and_update_callback.clone(),
                );

                let transform_feedback_link =
                    TransformFeedbackLink::new(TransformFeedbackId::Particle);

                // provide custom attributes when getting WebGL context
                let get_context_callback = GetContextCallback::new(CallbackWithContext::new(
                    Rc::new(|canvas: &HtmlCanvasElement| {
                        let mut webgl_context_attributes = WebGlContextAttributes::new();
                        webgl_context_attributes.preserve_drawing_buffer(true);

                        let gl = canvas
                            .get_context_with_context_options("webgl2", &webgl_context_attributes)
                            .map_err(|_| WebGlContextError::RetrievalError)?;

                        let gl = gl.ok_or(WebGlContextError::NotFoundError)?;

                        let gl: WebGl2RenderingContext = gl
                            .dyn_into()
                            .map_err(|_| WebGlContextError::TypeConversionError)?;

                        Ok(gl)
                    }),
                ));

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
                    .add_attribute_link(a_particle_position_link_a)
                    .add_attribute_link(a_particle_position_link_b)
                    .add_texture_link(perlin_noise_texture_link)
                    .add_texture_link(white_noise_texture_link)
                    .add_framebuffer_link(perlin_noise_framebuffer_link)
                    .add_uniform_link(u_perlin_noise_texture)
                    .add_uniform_link(u_white_noise_texture)
                    .add_uniform_link(u_now)
                    .add_transform_feedback_link(transform_feedback_link)
                    .add_vao_link(VAOId::PerlinNoise)
                    .add_vao_link(VAOId::PassThrough)
                    .add_vao_link(VAOId::UpdateParticlesA)
                    .add_vao_link(VAOId::UpdateParticlesB)
                    .add_vao_link(VAOId::DrawParticles)
                    .set_get_context_callback(get_context_callback);

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

    let handle_click = {
        let render_state = Rc::clone(&render_state);
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            render_state.borrow_mut().set_should_save_image(true);
        })
    };

    html! {
        <div class="flow-field">
            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            <button onclick={handle_click}>{"Save Image"}</button>
            <canvas ref={canvas_ref} height={1000} width={1000} />
        </div>
    }
}
