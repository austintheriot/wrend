use std::{cell::RefCell, rc::Rc};

use crate::{
    graphics::{
        create_framebuffer, create_position_attribute, create_vertex_buffer,
        make_crate_src_video_texture, make_create_render_texture, render, AttributeId, BufferId,
        FragmentShaderId, FramebufferId, ProgramId, TextureId, UniformId, VAOId, VertexShaderId,
    },
    state::{RenderState, RenderStateHandle},
};

use web_sys::HtmlCanvasElement;
use wrend::{
    AttributeLink, BufferLink, FramebufferLink, ProgramLinkBuilder, Renderer, RendererData,
    TextureLink, UniformContext, UniformLink,
};

use yew::NodeRef;

use super::TransformFeedbackId;

const QUAD_VERTEX_SHADER: &str = include_str!("../shaders/vertex.glsl");
const UNFILTERED_FRAGMENT_SHADER: &str = include_str!("../shaders/unfiltered.glsl");
const GRAYSCALE_FRAGMENT_SHADER: &str = include_str!("../shaders/grayscale.glsl");
const INVERT_FRAGMENT_SHADER: &str = include_str!("../shaders/invert.glsl");
const WAVY_FRAGMENT_SHADER: &str = include_str!("../shaders/wavy.glsl");

pub struct MakeInitializeArgs {
    pub canvas_ref: NodeRef,
    pub video_ref: NodeRef,
    pub render_state_handle_ref: Rc<RefCell<Option<RenderStateHandle>>>,
}

pub fn initialize_renderer(
    MakeInitializeArgs {
        canvas_ref,
        video_ref,
        render_state_handle_ref,
    }: MakeInitializeArgs,
) -> Renderer<
    VertexShaderId,
    FragmentShaderId,
    ProgramId,
    UniformId,
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

    let video = video_ref
        .cast()
        .expect("Video element was not ready for initialization");
    let render_state_handle: RenderStateHandle = RenderState::new(video).into();
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

        let mut wavy_program_link = ProgramLinkBuilder::new();
    wavy_program_link
        .set_vertex_shader_id(VertexShaderId::Quad)
        .set_program_id(ProgramId::Wavy)
        .set_fragment_shader_id(FragmentShaderId::Wavy);
    let wavy_program_link = wavy_program_link
        .build()
        .expect("Should build Wavy ProgramLink successfully");

    let mut invert_program_link = ProgramLinkBuilder::new();
    invert_program_link
        .set_vertex_shader_id(VertexShaderId::Quad)
        .set_program_id(ProgramId::Invert)
        .set_fragment_shader_id(FragmentShaderId::Invert);
    let invert_program_link = invert_program_link
        .build()
        .expect("Should build Invert ProgramLink successfully");

    let vertex_buffer_link = BufferLink::new(BufferId::QuadVertexBuffer, create_vertex_buffer);

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
        [ProgramId::Unfiltered, ProgramId::Grayscale, ProgramId::Invert, ProgramId::Wavy],
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

    let mut u_now_link = {
        UniformLink::new(
            ProgramId::Wavy,
            UniformId::UNow,
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
            FragmentShaderId::Grayscale,
            GRAYSCALE_FRAGMENT_SHADER.to_string(),
        )
        .add_fragment_shader_src(
            FragmentShaderId::Unfiltered,
            UNFILTERED_FRAGMENT_SHADER.to_string(),
        )
        .add_fragment_shader_src(
            FragmentShaderId::Invert,
            INVERT_FRAGMENT_SHADER.to_string(),
        )
        .add_fragment_shader_src(
            FragmentShaderId::Wavy,
            WAVY_FRAGMENT_SHADER.to_string(),
        )
        .add_program_link(unfiltered_program_link)
        .add_program_link(grayscale_program_link)
        .add_program_link(invert_program_link)
        .add_program_link(wavy_program_link)
        .add_buffer_link(vertex_buffer_link)
        .add_attribute_link(a_quad_vertex_link)
        .add_uniform_link(u_src_video_texture)
        .add_uniform_link(u_now_link)
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

    new_renderer
}
