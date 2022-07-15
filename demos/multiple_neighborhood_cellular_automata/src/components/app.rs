use crate::{
    graphics::{
        buffer_id::BufferId, create_buffer::create_vertex_buffer,
        create_framebuffer::make_create_frame_buffer, create_texture::create_texture,
        framebuffer_id::FramebufferId, program_id::ProgramId, render::render, shader_id::ShaderId,
        texture_id::TextureId, uniform_id::UniformId,
    },
    state::render_state::RenderState,
};
use std::rc::Rc;
use ui::route::Route;
use web_sys::HtmlCanvasElement;
use webgl::renderer::{
    animation_callback::AnimationCallback, buffer_link::BufferLink,
    framebuffer_link::FramebufferLink, program_link::ProgramLink, render_callback::RenderCallback,
    renderer::Renderer, texture_link::TextureLink, uniform_link::UniformLink, uniform_callback::UniformCallback,
};
use yew::{function_component, html, use_effect_with_deps, use_mut_ref, use_node_ref};
use yew_router::prelude::*;

const VERTEX_SHADER: &'static str = include_str!("../shaders/vertex.glsl");
const GAME_OF_LIFE_FRAGMENT_SHADER: &'static str = include_str!("../shaders/game_of_life.glsl");
const PASS_THROUGH_FRAGMENT_SHADER: &'static str = include_str!("../shaders/pass_through.glsl");

#[function_component(App)]
pub fn app() -> Html {
    let canvas_ref = use_node_ref();
    let render_state = use_mut_ref(RenderState::default);
    let animation_handle = use_mut_ref(|| None);

    use_effect_with_deps(
        {
            let canvas_ref = canvas_ref.clone();
            let animation_handle = animation_handle.clone();
            move |_| {
                let canvas: HtmlCanvasElement = canvas_ref
                    .cast()
                    .expect("Canvas ref should point to a canvas in the use_effect hook");

                let game_of_life_program_link =
                    ProgramLink::new(ProgramId::GameOfLife, ShaderId::Vertex, ShaderId::GameOfLife);

                    let pass_through_program_link =
                    ProgramLink::new(ProgramId::PassThrough, ShaderId::Vertex, ShaderId::PassThrough);

                let a_position_link = BufferLink::new(
                    ProgramId::GameOfLife,
                    BufferId::VertexBuffer,
                    Rc::new(create_vertex_buffer),
                    Rc::new(|_| {}),
                    Rc::new(|_| false),
                );

                let u_texture = UniformLink::new(
                    ProgramId::GameOfLife,
                    UniformId::UTexture,
                    UniformCallback::new(Rc::new(|ctx| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1i(Some(uniform_location), 0);
                    })),
                );

                let texture_a_link =
                    TextureLink::new(ProgramId::GameOfLife, TextureId::A, Rc::new(create_texture));

                let texture_b_link =
                    TextureLink::new(ProgramId::GameOfLife, TextureId::B, Rc::new(create_texture));

                let framebuffer_a_link = FramebufferLink::new(
                    ProgramId::GameOfLife,
                    FramebufferId::A,
                    make_create_frame_buffer(TextureId::A),
                );

                let framebuffer_b_link = FramebufferLink::new(
                    ProgramId::GameOfLife,
                    FramebufferId::B,
                    make_create_frame_buffer(TextureId::B),
                );

                let render_callback = RenderCallback::new(Rc::new(render));

                let mut renderer_builder = Renderer::builder();

                renderer_builder
                    .set_canvas(canvas)
                    .set_user_ctx(render_state)
                    .set_render_callback(render_callback)
                    .add_vertex_shader_src(ShaderId::Vertex, VERTEX_SHADER.to_string())
                    .add_fragment_shader_src(ShaderId::GameOfLife, GAME_OF_LIFE_FRAGMENT_SHADER.to_string())
                    .add_fragment_shader_src(ShaderId::PassThrough, PASS_THROUGH_FRAGMENT_SHADER.to_string())
                    .add_program_link(game_of_life_program_link)
                    .add_program_link(pass_through_program_link)
                    .add_buffer_link(a_position_link)
                    .add_uniform_link(u_texture)
                    .add_texture_link(texture_a_link)
                    .add_texture_link(texture_b_link)
                    .add_framebuffer_link(framebuffer_a_link)
                    .add_framebuffer_link(framebuffer_b_link);

                let renderer = renderer_builder
                    .build()
                    .expect("Renderer should successfully build");

                let new_animation_handle =
                    renderer.into_animation_handle(AnimationCallback::new(Rc::new(|renderer| {
                        renderer.render();
                    })));

                new_animation_handle.start_animating();

                // save handle to keep animation going
                *animation_handle.borrow_mut() = Some(new_animation_handle);

                return || {};
            }
        },
        (),
    );

    html! {
        <div class="multiple-neighborhood-cellular-automata">
            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            <canvas ref={canvas_ref} height={1000} width={1000} />
        </div>
    }
}
