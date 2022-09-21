use crate::{
    graphics::{
        create_framebuffer, create_position_attribute, create_vertex_buffer,
        make_crate_src_video_texture, make_create_render_texture, render, AttributeId, BufferId,
        FilterType, FragmentShaderId, FramebufferId, ProgramId, TextureId, UniformId, VAOId,
        VertexShaderId,
    },
    state::{RenderState, RenderStateHandle},
};

use log::{error, info};
use shared::route::Route;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlCanvasElement, HtmlSelectElement};
use wrend::{
    AttributeLink, BufferLink, FramebufferLink, ProgramLinkBuilder, RendererData, TextureLink,
    UniformContext, UniformLink,
};

use yew::{function_component, html, use_effect_with_deps, use_mut_ref, use_node_ref, Callback};
use yew_router::prelude::*;

const QUAD_VERTEX_SHADER: &str = include_str!("../shaders/vertex.glsl");
const UNFILTERED_FRAGMENT_SHADER: &str = include_str!("../shaders/unfiltered.glsl");
const GRAYSCALE_FRAGMENT_SHADER: &str = include_str!("../shaders/grayscale.glsl");

#[function_component(App)]
pub fn app() -> Html {
    let canvas_ref = use_node_ref();
    let video_ref = use_node_ref();
    let select_ref = use_node_ref();
    let render_state_handle_ref = use_mut_ref(|| None);
    let renderer_ref = use_mut_ref(|| None);

    use_effect_with_deps(
        {
            let canvas_ref = canvas_ref.clone();
            let video_ref = video_ref.clone();
            let render_state_handle_ref = render_state_handle_ref.clone();
            move |_| {
                let canvas: HtmlCanvasElement = canvas_ref
                    .cast()
                    .expect("Canvas ref should point to a canvas in the use_effect hook");

                let video = video_ref
                    .cast()
                    .expect("Video element was not ready for initialization");
                let render_state = RenderState::new(video);
                let render_state_handle: RenderStateHandle = render_state.into();
                render_state_handle_ref.replace(Some(render_state_handle.clone()));

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
                    .expect("Should build Grayscale ProgramLink successfully");

                let vertex_buffer_link =
                    BufferLink::new(BufferId::QuadVertexBuffer, create_vertex_buffer);

                let a_quad_vertex_link = AttributeLink::new(
                    VAOId::Quad,
                    BufferId::QuadVertexBuffer,
                    AttributeId,
                    create_position_attribute,
                );

                let src_video_texture_link = TextureLink::new(
                    TextureId::SrcVideo,
                    make_crate_src_video_texture(render_state_handle.clone()),
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
                    [ProgramId::Unfiltered, ProgramId::Grayscale],
                    UniformId::USrcVideoTexture,
                    |ctx: &UniformContext| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1i(
                            Some(uniform_location),
                            TextureId::SrcVideo.location() as i32,
                        );
                    },
                );

                let mut renderer_data_builder = RendererData::builder();

                renderer_data_builder
                    .set_canvas(canvas)
                    .set_user_ctx(render_state_handle)
                    .set_render_callback(render)
                    .add_vertex_shader_src(VertexShaderId::Quad, QUAD_VERTEX_SHADER.to_string())
                    .add_fragment_shader_src(
                        FragmentShaderId::Grayscale,
                        GRAYSCALE_FRAGMENT_SHADER.to_string(),
                    )
                    .add_fragment_shader_src(
                        FragmentShaderId::Unfiltered,
                        UNFILTERED_FRAGMENT_SHADER.to_string(),
                    )
                    .add_program_link(unfiltered_program_link)
                    .add_program_link(grayscale_program_link)
                    .add_buffer_link(vertex_buffer_link)
                    .add_attribute_link(a_quad_vertex_link)
                    .add_uniform_link(u_src_video_texture)
                    .add_texture_link(src_video_texture_link)
                    .add_texture_link(prev_render_texture_link_a)
                    .add_texture_link(prev_render_texture_link_b)
                    .add_framebuffer_link(prev_render_framebuffer_link_a)
                    .add_framebuffer_link(prev_render_framebuffer_link_b)
                    .add_vao_link(VAOId::Quad);

                let mut new_renderer = renderer_data_builder
                    .build_renderer()
                    .expect("RendererData should successfully build");

                new_renderer.set_animation_callback(Some(
                    |renderer_data: &RendererData<_, _, _, _, _, _, _, _, _, _, _>| {
                        renderer_data.update_uniforms();
                        renderer_data.render();
                    },
                ));

                new_renderer.start_animating();

                // save handle to keep animation going
                *renderer_ref.borrow_mut() = Some(new_renderer);

                || {}
            }
        },
        (),
    );

    let handle_change = {
        let render_state_handle_ref = render_state_handle_ref.clone();
        let select_ref = select_ref.clone();
        Callback::from(move |_: Event| {
            if let Some(render_state_handle) = render_state_handle_ref.borrow().as_ref() {
                let select_element = select_ref
                    .get()
                    .unwrap()
                    .dyn_into::<HtmlSelectElement>()
                    .unwrap();
                let selected_index = select_element.selected_index();

                let filter_type = match FilterType::iter().skip(selected_index as usize).next() {
                    Some(filter_type) => filter_type,
                    None => {
                        error!("Unexpected select option reached: index =  {selected_index}");
                        Default::default()
                    }
                };

                render_state_handle
                    .borrow_mut()
                    .set_filter_type(filter_type);
            }
        })
    };

    html! {
        <div class="video-filters">
            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            <video controls=true ref={video_ref} src="./big_buck_bunny.mp4" />
            <canvas ref={canvas_ref}  />
            <label for="select-filter">{"Choose a filter"}</label>
            <select
                name="filter"
                id="select-filter"
                onchange={handle_change}
                ref={select_ref}
            >
                {for FilterType::iter().map(|filter_type| {
                    html!{
                        <option 
                            value={filter_type.to_string()} 
                            selected={filter_type == FilterType::default()}
                        >
                            {filter_type.to_string()}
                        </option>
                    }
                })}
            </select>

        </div>
    }
}
