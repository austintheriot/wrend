use std::{cell::RefCell, rc::Rc};

use crate::{
    graphics::{
        create_framebuffer, create_position_attribute, create_vertex_buffer,
        make_create_src_texture, make_create_render_texture, render, AttributeId, BufferId,
        FragmentShaderId, FramebufferId, ProgramId, TextureId, UniformId, VAOId, VertexShaderId,
    },
    state::{RenderState, RenderStateHandle},
};

use strum::IntoEnumIterator;
use web_sys::HtmlCanvasElement;
use wrend::{
    AttributeLink, BufferLink, FramebufferLink, IdName, Renderer, RendererData, TextureLink,
    UniformContext, UniformLink,
};

use yew::NodeRef;

use super::{create_program_links, FilterType, TransformFeedbackId};

const QUAD_VERTEX_SHADER: &str = include_str!("../shaders/vertex.glsl");
const GENERATE_CIRCLE_GRADIENT: &str = include_str!("../shaders/generate_circle_gradient.glsl");
const FILTER_UNFILTERED_FRAGMENT_SHADER: &str = include_str!("../shaders/filter_unfiltered.glsl");
const FILTER_SPLIT_FRAGMENT_SHADER: &str = include_str!("../shaders/filter_split.glsl");

pub struct InitializeRendererArgs {
    pub canvas_ref: NodeRef,
    pub render_state_handle_ref: Rc<RefCell<Option<RenderStateHandle>>>,
}

pub fn initialize_renderer(
    InitializeRendererArgs {
        canvas_ref,
        render_state_handle_ref,
    }: InitializeRendererArgs,
) -> Renderer<
    VertexShaderId,
    FragmentShaderId,
    ProgramId,
    String,
    BufferId,
    AttributeId,
    TextureId,
    FramebufferId,
    TransformFeedbackId,
    VAOId,
    RenderStateHandle,
> {
    let canvas: HtmlCanvasElement = canvas_ref
        .cast()
        .expect("Canvas ref should point to a canvas in the use_effect hook");

    let render_state_handle: RenderStateHandle = RenderState::new().into();
    render_state_handle_ref.replace(Some(render_state_handle.clone()));

    let program_links = create_program_links();

    let vertex_buffer_link = BufferLink::new(BufferId::QuadVertexBuffer, create_vertex_buffer);

    let a_quad_vertex_link = AttributeLink::new(
        VAOId::Quad,
        BufferId::QuadVertexBuffer,
        AttributeId,
        create_position_attribute,
    );

    let src_texture_link = TextureLink::new(
        TextureId::SrcTexture,
        make_create_src_texture(render_state_handle.clone()),
    );

    let prev_render_texture_link_a = TextureLink::new(
        TextureId::PrevRenderA,
        make_create_render_texture(render_state_handle.clone(), TextureId::PrevRenderA),
    );

    let prev_render_texture_link_b = TextureLink::new(
        TextureId::PrevRenderB,
        make_create_render_texture(render_state_handle.clone(), TextureId::PrevRenderB),
    );

    let src_texture_framebuffer_link = FramebufferLink::new(
        FramebufferId::SrcTexture,
        create_framebuffer,
        Some(TextureId::SrcTexture),
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

    // it's safe to assume for now that we may need the source texture for every Filter
    let program_ids_for_u_src_texture = FilterType::iter()
        .map(|filter_type| filter_type.program_id())
        .collect::<Vec<_>>();
    let u_src_texture = UniformLink::new(
        program_ids_for_u_src_texture,
        UniformId::USrcTexture.name(),
        |ctx: &UniformContext| {
            let gl = ctx.gl();
            let uniform_location = ctx.uniform_location();
            gl.uniform1i(
                Some(uniform_location),
                TextureId::SrcTexture.location() as i32,
            );
        },
    );

    let mut u_now_link = {
        UniformLink::new(
            [],
            UniformId::UNow.name(),
            |ctx: &UniformContext| {
                let gl = ctx.gl();
                let uniform_location = ctx.uniform_location();
                gl.uniform1f(Some(uniform_location), (ctx.now() / 2000.) as f32);
            },
        )
    };
    u_now_link.set_use_init_callback_for_update(true);

    let mut renderer_data_builder = RendererData::builder();

    renderer_data_builder
        .set_canvas(canvas)
        .set_user_ctx(render_state_handle)
        .set_render_callback(render)
        .add_vertex_shader_src(VertexShaderId::Quad, QUAD_VERTEX_SHADER.to_string())
        .add_fragment_shader_src(
            FragmentShaderId::GenerateCircleGradient,
            GENERATE_CIRCLE_GRADIENT.to_string(),
        )
        .add_fragment_shader_src(
            FragmentShaderId::FilterSplit,
            FILTER_SPLIT_FRAGMENT_SHADER.to_string(),
        )
        .add_fragment_shader_src(
            FragmentShaderId::FilterUnfiltered,
            FILTER_UNFILTERED_FRAGMENT_SHADER.to_string(),
        )
        .add_program_links(program_links)
        .add_buffer_link(vertex_buffer_link)
        .add_attribute_link(a_quad_vertex_link)
        .add_uniform_link(u_src_texture)
        .add_uniform_link(u_now_link)
        .add_texture_link(src_texture_link)
        .add_texture_link(prev_render_texture_link_a)
        .add_texture_link(prev_render_texture_link_b)
        .add_framebuffer_link(prev_render_framebuffer_link_a)
        .add_framebuffer_link(prev_render_framebuffer_link_b)
        .add_framebuffer_link(src_texture_framebuffer_link)
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

    new_renderer
}
