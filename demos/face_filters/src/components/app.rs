use crate::{
    graphics::{
        create_framebuffer, create_position_attribute, create_vertex_buffer,
        make_create_render_texture, render, AttributeId, BufferId, FragmentShaderId, FramebufferId,
        ProgramId, TextureId, UniformId, VertexShaderId,
    },
    state::{RenderState, RenderStateHandle},
};

use shared::route::Route;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, HtmlVideoElement};
use wrend::{
    AttributeLink, BufferLink, FramebufferLink, ProgramLinkBuilder, RendererData, TextureLink,
    UniformContext, UniformLink,
};

use yew::{function_component, html, use_effect_with_deps, use_mut_ref, use_node_ref};
use yew_router::prelude::*;

const VERTEX_SHADER: &str = include_str!("../shaders/vertex.glsl");
const PASS_THROUGH_FRAGMENT_SHADER: &str = include_str!("../shaders/pass_through.glsl");
const PERLIN_NOISE_FRAGMENT_SHADER: &str = include_str!("../shaders/face_filters.glsl");

#[function_component(App)]
pub fn app() -> Html {
    let canvas_ref = use_node_ref();
    let video_ref = use_node_ref();
    let render_state = use_mut_ref(|| {
        let video = video_ref
            .cast()
            .expect("Video element was not ready for initialization");
        RenderState::new(video)
    });
    let renderer = use_mut_ref(|| None);

    use_effect_with_deps(
        {
            let canvas_ref = canvas_ref.clone();
            let renderer = renderer;
            move |_| {
                let canvas: HtmlCanvasElement = canvas_ref
                    .cast()
                    .expect("Canvas ref should point to a canvas in the use_effect hook");

                let render_state_handle: RenderStateHandle = render_state.into();

                let mut unfiltered_program_link = ProgramLinkBuilder::new();
                unfiltered_program_link
                    .set_vertex_shader_id(VertexShaderId::Quad)
                    .set_program_id(ProgramId::Unfiltered)
                    .set_fragment_shader_id(FragmentShaderId::Unfiltered);
                let unfiltered_program_link = unfiltered_program_link
                    .build()
                    .expect("Should build Unfiltered ProgramLink successfully");

                let mut grayscale_program_link = ProgramLinkBuilder::new();
                grayscale_program_link
                    .set_vertex_shader_id(VertexShaderId::Quad)
                    .set_program_id(ProgramId::Grayscale)
                    .set_fragment_shader_id(FragmentShaderId::Grayscale);
                let grayscale_program_link = grayscale_program_link
                    .build()
                    .expect("Should build Unfiltered Grayscale successfully");

                let vertex_buffer_link =
                    BufferLink::new(BufferId::QuadVertexBuffer, create_vertex_buffer);

                let a_quad_vertex_link = AttributeLink::new(
                    (ProgramId::Unfiltered, ProgramId::Grayscale),
                    BufferId::QuadVertexBuffer,
                    AttributeId,
                    create_position_attribute,
                );

                let prev_render_texture_link_a = TextureLink::new(
                    TextureId::PrevRenderA,
                    make_create_render_texture(render_state_handle.clone(), TextureId::PrevRenderA),
                );

                let prev_render_texture_link_b = TextureLink::new(
                    TextureId::PrevRenderB,
                    make_create_render_texture(render_state_handle.clone(), TextureId::PrevRenderB),
                );

                let prev_render_framebuffer_link_a = FramebufferLink::new(
                    FramebufferId::PrevRenderA,
                    create_framebuffer,
                    Some(TextureId::PrevRenderA),
                );

                let prev_render_framebuffer_link_b = FramebufferLink::new(
                    FramebufferId::PrevRenderB,
                    create_framebuffer,
                    Some(TextureId::PrevRenderB),
                );

                let u_src_video_texture = UniformLink::new(
                    ProgramId::Unfiltered,
                    UniformId::USrcVideoTexture,
                    |ctx: &UniformContext| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1i(Some(uniform_location), 1);
                    },
                );

                let mut u_now = UniformLink::new(
                    ProgramId::Grayscale,
                    UniformId::UNow,
                    |ctx: &UniformContext| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1f(Some(uniform_location), (ctx.now() / 2000.) as f32);
                    },
                );
                u_now.set_use_init_callback_for_update(true);

                let mut renderer_data_builder = RendererData::builder();

                renderer_data_builder
                    .set_canvas(canvas)
                    .set_user_ctx(render_state_handle)
                    .set_render_callback(render)
                    .add_vertex_shader_src(VertexShaderId::Quad, VERTEX_SHADER.to_string())
                    .add_fragment_shader_src(
                        FragmentShaderId::Grayscale,
                        PERLIN_NOISE_FRAGMENT_SHADER.to_string(),
                    )
                    .add_fragment_shader_src(
                        FragmentShaderId::Unfiltered,
                        PASS_THROUGH_FRAGMENT_SHADER.to_string(),
                    )
                    .add_program_link(unfiltered_program_link)
                    .add_program_link(grayscale_program_link)
                    .add_buffer_link(vertex_buffer_link)
                    .add_attribute_link(a_quad_vertex_link)
                    .add_uniform_link(u_now)
                    .add_uniform_link(u_src_video_texture)
                    .add_texture_link(prev_render_texture_link_a)
                    .add_texture_link(prev_render_texture_link_b)
                    .add_framebuffer_link(prev_render_framebuffer_link_a)
                    .add_framebuffer_link(prev_render_framebuffer_link_b)
                    .add_vao_link(ProgramId::Unfiltered)
                    .add_vao_link(ProgramId::Grayscale);

                let renderer_data = renderer_data_builder
                    .build_renderer_data()
                    .expect("RendererData should successfully build");

                let mut new_renderer = renderer_data.into_renderer();
                new_renderer.set_animation_callback(Some(
                    |renderer_data: &RendererData<_, _, _, _, _, _, _, _, _, _, _>| {
                        renderer_data.update_uniforms();
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

    html! {
        <div class="face-filters">
            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            <video controls=true ref={video_ref} src="https://archive.org/download/BigBuckBunny_124/Content/big_buck_bunny_720p_surround.mp4" />
            <canvas ref={canvas_ref} height={1000} width={1000} />
        </div>
    }
}
