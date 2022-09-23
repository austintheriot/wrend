use std::{cell::RefCell, rc::Rc};

use crate::{
    graphics::{
        build_gaussian_kernel, create_framebuffer, create_position_attribute, create_vertex_buffer,
        make_crate_src_video_texture, make_create_render_texture, render, AttributeId, BufferId,
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
const UNFILTERED_FRAGMENT_SHADER: &str = include_str!("../shaders/unfiltered.glsl");
const GRAYSCALE_FRAGMENT_SHADER: &str = include_str!("../shaders/grayscale.glsl");
const INVERT_FRAGMENT_SHADER: &str = include_str!("../shaders/invert.glsl");
const WAVY_FRAGMENT_SHADER: &str = include_str!("../shaders/wavy.glsl");
const GAUSSIAN_BLUR_FRAGMENT_SHADER: &str = include_str!("../shaders/gaussian_blur.glsl");

pub struct InitializeRendererArgs {
    pub canvas_ref: NodeRef,
    pub video_ref: NodeRef,
    pub render_state_handle_ref: Rc<RefCell<Option<RenderStateHandle>>>,
}

pub fn initialize_renderer(
    InitializeRendererArgs {
        canvas_ref,
        video_ref,
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

    let video = video_ref
        .cast()
        .expect("Video element was not ready for initialization");
    let render_state_handle: RenderStateHandle = RenderState::new(video).into();
    render_state_handle_ref.replace(Some(render_state_handle.clone()));

    let program_links = create_program_links();

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

    // it's safe to assume for now that we'll need the source video for every Filter
    let program_ids_for_u_src_video_texture = FilterType::iter()
        .map(|filter_type| filter_type.program_id())
        .collect::<Vec<_>>();
    let u_src_video_texture = UniformLink::new(
        program_ids_for_u_src_video_texture,
        UniformId::USrcVideoTexture.name(),
        |ctx: &UniformContext| {
            let gl = ctx.gl();
            let uniform_location = ctx.uniform_location();
            gl.uniform1i(
                Some(uniform_location),
                TextureId::SrcVideo.location() as i32,
            );
        },
    );

    // set all Gaussian Blur kernel element uniforms
    const GAUSSIAN_KERNEL_SIZE: usize = 9;
    let gaussian_kernel = Rc::new(build_gaussian_kernel(GAUSSIAN_KERNEL_SIZE));
    let mut kernel_element_uniform_links = Vec::with_capacity(GAUSSIAN_KERNEL_SIZE);
    for i in 0..GAUSSIAN_KERNEL_SIZE {
        let u_kernel_element_link = {
            let gaussian_kernel = Rc::clone(&gaussian_kernel);
            UniformLink::new(
                ProgramId::GaussianBlur,
                format!("u_kernel[{}]", i),
                move |ctx: &UniformContext| {
                    let gl = ctx.gl();
                    let uniform_location = ctx.uniform_location();
                    gl.uniform1f(Some(uniform_location), gaussian_kernel[i]);
                },
            )
        };
        kernel_element_uniform_links.push(u_kernel_element_link);
    }

    let mut u_now_link = {
        UniformLink::new(
            ProgramId::Wavy,
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
            FragmentShaderId::Grayscale,
            GRAYSCALE_FRAGMENT_SHADER.to_string(),
        )
        .add_fragment_shader_src(
            FragmentShaderId::Unfiltered,
            UNFILTERED_FRAGMENT_SHADER.to_string(),
        )
        .add_fragment_shader_src(FragmentShaderId::Invert, INVERT_FRAGMENT_SHADER.to_string())
        .add_fragment_shader_src(FragmentShaderId::Wavy, WAVY_FRAGMENT_SHADER.to_string())
        .add_fragment_shader_src(
            FragmentShaderId::GaussianBlur,
            GAUSSIAN_BLUR_FRAGMENT_SHADER.to_string(),
        )
        .add_program_links(program_links)
        .add_buffer_link(vertex_buffer_link)
        .add_attribute_link(a_quad_vertex_link)
        .add_uniform_links(kernel_element_uniform_links)
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
