use std::rc::Rc;

use crate::{
    graphics::{
        attribute_id::AttributeId, buffer_id::BufferId, create_buffer::create_vertex_buffer,
        create_framebuffer::create_frame_buffer,
        create_position_attribute::create_position_attribute, create_texture::create_texture,
        fragment_shader_id::FragmentShaderId, framebuffer_id::FramebufferId, program_id::ProgramId,
        render::render, texture_id::TextureId, uniform_id::UniformId,
        vertex_shader_id::VertexShaderId,
    },
    state::render_state::RenderState,
};

use shared::{route::Route, SharedClass};
use web_sys::{HtmlCanvasElement, MouseEvent};
use wrend::{
    AttributeLink, BufferLink, FramebufferLink, ProgramLink, RendererData, TextureLink,
    UniformContext, UniformLink,
};
use yew::{function_component, html, use_effect_with_deps, use_mut_ref, use_node_ref, classes, use_state_eq, Callback};
use yew_router::prelude::*;

const VERTEX_SHADER: &str = include_str!("../shaders/vertex.glsl");
const GAME_OF_LIFE_FRAGMENT_SHADER: &str = include_str!("../shaders/game_of_life.glsl");
const PASS_THROUGH_FRAGMENT_SHADER: &str = include_str!("../shaders/pass_through.glsl");

#[function_component(App)]
pub fn app() -> Html {
    let canvas_ref = use_node_ref();
    let render_state = use_mut_ref(RenderState::default);
    let renderer = use_mut_ref(|| None);
    let is_recording = use_state_eq(bool::default);

    use_effect_with_deps(
        {
            let canvas_ref = canvas_ref.clone();
            let renderer = Rc::clone(&renderer);
            move |_| {
                let canvas: HtmlCanvasElement = canvas_ref
                    .cast()
                    .expect("Canvas ref should point to a canvas in the use_effect hook");

                let game_of_life_program_link = ProgramLink::new(
                    ProgramId::GameOfLife,
                    VertexShaderId,
                    FragmentShaderId::GameOfLife,
                );

                let pass_through_program_link = ProgramLink::new(
                    ProgramId::PassThrough,
                    VertexShaderId,
                    FragmentShaderId::PassThrough,
                );

                let vertex_buffer_link =
                    BufferLink::new(BufferId::VertexBuffer, create_vertex_buffer);

                let a_position_gol_life = AttributeLink::new(
                    (ProgramId::GameOfLife, ProgramId::PassThrough),
                    BufferId::VertexBuffer,
                    AttributeId,
                    create_position_attribute,
                );

                let u_texture = UniformLink::new(
                    (ProgramId::GameOfLife, ProgramId::PassThrough),
                    UniformId::UTexture,
                    |ctx: &UniformContext| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1i(Some(uniform_location), 0);
                    },
                );

                let texture_a_link = TextureLink::new(TextureId::A, create_texture);

                let texture_b_link = TextureLink::new(TextureId::B, create_texture);

                let framebuffer_a_link =
                    FramebufferLink::new(FramebufferId::A, create_frame_buffer, Some(TextureId::A));

                let framebuffer_b_link =
                    FramebufferLink::new(FramebufferId::B, create_frame_buffer, Some(TextureId::B));

                let mut renderer_data_builder = RendererData::builder();

                renderer_data_builder
                    .set_canvas(canvas)
                    .set_user_ctx(render_state)
                    .set_render_callback(render)
                    .add_vertex_shader_src(VertexShaderId, VERTEX_SHADER.to_string())
                    .add_fragment_shader_src(
                        FragmentShaderId::GameOfLife,
                        GAME_OF_LIFE_FRAGMENT_SHADER.to_string(),
                    )
                    .add_fragment_shader_src(
                        FragmentShaderId::PassThrough,
                        PASS_THROUGH_FRAGMENT_SHADER.to_string(),
                    )
                    .add_program_link(game_of_life_program_link)
                    .add_program_link(pass_through_program_link)
                    .add_buffer_link(vertex_buffer_link)
                    .add_attribute_link(a_position_gol_life)
                    .add_uniform_link(u_texture)
                    .add_texture_link(texture_a_link)
                    .add_texture_link(texture_b_link)
                    .add_framebuffer_link(framebuffer_a_link)
                    .add_framebuffer_link(framebuffer_b_link)
                    .add_vao_link(ProgramId::GameOfLife)
                    .add_vao_link(ProgramId::PassThrough);

                let renderer_data = renderer_data_builder
                    .build_renderer_data()
                    .expect("RendererData should successfully build");

                let mut new_renderer = renderer_data.into_renderer();
                new_renderer.set_animation_callback(Some(
                    |renderer_data: &RendererData<_, _, _, _, _, _, _, _, _, _, _>| {
                        renderer_data.render();
                    },
                ));

                new_renderer.start_animating();

                // save handle to keep animation going
                *renderer.borrow_mut() = Some(new_renderer);

                || {}
            }
        },
        (),
    );

    let handle_start_recording = {
        let renderer = Rc::clone(&renderer);
        let is_recording = is_recording.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(renderer) = &mut *renderer.borrow_mut() {
                renderer.start_recording();
                is_recording.set(true);
            }
        })
    };

    let handle_stop_recording = {
        let renderer = Rc::clone(&renderer);
        let is_recording = is_recording.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(renderer) = &*renderer.borrow() {
                renderer.stop_recording();
                is_recording.set(false);
            }
        })
    };

    html! {
        <div class="game-of-life">
            <div class="ui-container">
                <Link<Route> to={Route::Home} classes={classes!(SharedClass::Button.to_string())}>{"Home"}</Link<Route>>
                {if !*is_recording {
                    html!{
                        <button type="button" onclick={handle_start_recording} class={SharedClass::Button.to_string()}>
                            {"Start Recording"}
                        </button>
                    }
                } else {
                    html!{
                        <button type="button" onclick={handle_stop_recording} class={SharedClass::Button.to_string()}>
                            {"Stop Recording"}
                        </button>
                    }
                }}
            </div>
            <canvas ref={canvas_ref} height={250} width={250} />
        </div>
    }
}
