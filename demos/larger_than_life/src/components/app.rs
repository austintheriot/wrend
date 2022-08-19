use crate::{
    graphics::{
        attribute_id::AttributeId, buffer_id::BufferId, create_buffer::create_vertex_buffer,
        create_framebuffer::create_frame_buffer,
        create_position_attribute::create_position_attribute, create_texture::create_texture,
        framebuffer_id::FramebufferId, program_id::ProgramId, render::render, shader_id::ShaderId,
        texture_id::TextureId, uniform_id::UniformId,
    },
    state::render_state::RenderState,
};

use ui::route::Route;
use web_sys::HtmlCanvasElement;
use wrend::{
    AttributeLink, BufferLink, FramebufferLink, ProgramLink, Renderer, TextureLink, UniformContext,
    UniformLink,
};

use yew::{function_component, html, use_effect_with_deps, use_mut_ref, use_node_ref};
use yew_router::prelude::*;

const VERTEX_SHADER: &str = include_str!("../shaders/vertex.glsl");
const GAME_OF_LIFE_FRAGMENT_SHADER: &str = include_str!("../shaders/game_of_life.glsl");
const PASS_THROUGH_FRAGMENT_SHADER: &str = include_str!("../shaders/pass_through.glsl");

#[function_component(App)]
pub fn app() -> Html {
    let canvas_ref = use_node_ref();
    let render_state = use_mut_ref(RenderState::default);
    let renderer_handle = use_mut_ref(|| None);

    use_effect_with_deps(
        {
            let canvas_ref = canvas_ref.clone();
            let renderer_handle = renderer_handle;
            move |_| {
                let canvas: HtmlCanvasElement = canvas_ref
                    .cast()
                    .expect("Canvas ref should point to a canvas in the use_effect hook");

                let game_of_life_program_link = ProgramLink::new(
                    ProgramId::GameOfLife,
                    ShaderId::Vertex,
                    ShaderId::GameOfLife,
                );

                let pass_through_program_link = ProgramLink::new(
                    ProgramId::PassThrough,
                    ShaderId::Vertex,
                    ShaderId::PassThrough,
                );

                let vertex_buffer_link =
                    BufferLink::new(BufferId::VertexBuffer, create_vertex_buffer);

                let attribute_position_gol_link = AttributeLink::new(
                    (ProgramId::GameOfLife, ProgramId::PassThrough),
                    BufferId::VertexBuffer,
                    AttributeId::APosition,
                    create_position_attribute,
                );

                let u_texture = UniformLink::new(
                    (ProgramId::GameOfLife, ProgramId::PassThrough),
                    UniformId::UTexture,
                    |ctx: &UniformContext<_>| {
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

                let mut renderer_builder = Renderer::builder();

                renderer_builder
                    .set_canvas(canvas)
                    .set_user_ctx(render_state)
                    .set_render_callback(render)
                    .add_vertex_shader_src(ShaderId::Vertex, VERTEX_SHADER.to_string())
                    .add_fragment_shader_src(
                        ShaderId::GameOfLife,
                        GAME_OF_LIFE_FRAGMENT_SHADER.to_string(),
                    )
                    .add_fragment_shader_src(
                        ShaderId::PassThrough,
                        PASS_THROUGH_FRAGMENT_SHADER.to_string(),
                    )
                    .add_program_link(game_of_life_program_link)
                    .add_program_link(pass_through_program_link)
                    .add_buffer_link(vertex_buffer_link)
                    .add_attribute_link(attribute_position_gol_link)
                    .add_uniform_link(u_texture)
                    .add_texture_link(texture_a_link)
                    .add_texture_link(texture_b_link)
                    .add_framebuffer_link(framebuffer_a_link)
                    .add_framebuffer_link(framebuffer_b_link)
                    .add_vao_link(ProgramId::PassThrough)
                    .add_vao_link(ProgramId::GameOfLife);

                let renderer = renderer_builder
                    .build()
                    .expect("Renderer should successfully build");

                let mut new_renderer_handle = renderer.into_renderer_handle();
                new_renderer_handle.set_animation_callback(Some(
                    |renderer: &Renderer<_, _, _, _, _, _, _, _, _, _, _>| {
                        renderer.render();
                    },
                ));

                new_renderer_handle.start_animating();

                // save handle to keep animation going
                *renderer_handle.borrow_mut() = Some(new_renderer_handle);

                || {}
            }
        },
        (),
    );

    html! {
        <div class="larger-than-life">
            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            <canvas ref={canvas_ref} height={500} width={500} />
        </div>
    }
}
