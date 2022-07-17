use crate::{
    graphics::{
        attribute_id::AttributeId, buffer_id::BufferId, create_buffer::create_vertex_buffer,
        create_position_attribute::create_position_attribute, create_texture::create_texture,
        fragment_shader_id::FragmentShaderId, program_id::ProgramId, render::render,
        texture_id::TextureId, uniform_id::UniformId, vertex_shader_id::VertexShaderId,
    },
    state::render_state::RenderState,
};
use std::rc::Rc;
use ui::route::Route;
use web_sys::HtmlCanvasElement;
use wrend::{
    AnimationCallback, AttributeLink, BufferCreateCallback, BufferLink, ProgramLinkBuilder,
    RenderCallback, Renderer, TextureLink, UniformCallback, UniformLink,
};

use yew::{function_component, html, use_effect_with_deps, use_mut_ref, use_node_ref};
use yew_router::prelude::*;

const VERTEX_SHADER: &str = include_str!("../shaders/vertex.glsl");
const FLOW_FIELD_FRAGMENT_SHADER: &str = include_str!("../shaders/flow_field.glsl");
const PASS_THROUGH_FRAGMENT_SHADER: &str = include_str!("../shaders/pass_through.glsl");

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

                let flow_field_program_link = ProgramLinkBuilder::new()
                    .set_vertex_shader_id(VertexShaderId::Quad)
                    .set_program_id(ProgramId::FlowField)
                    .set_fragment_shader_id(FragmentShaderId::FlowField)
                    .build()
                    .expect("Should build FlowField ProgramLink successfully");

                let pass_through_program_link = ProgramLinkBuilder::new()
                    .set_vertex_shader_id(VertexShaderId::Quad)
                    .set_program_id(ProgramId::PassThrough)
                    .set_fragment_shader_id(FragmentShaderId::PassThrough)
                    .build()
                    .expect("Should build PassThrough ProgramLink successfully");

                let vertex_buffer_link = BufferLink::new(
                    BufferId::VertexBuffer,
                    BufferCreateCallback::new(Rc::new(create_vertex_buffer)),
                );

                let a_position_link = AttributeLink::new(
                    ProgramId::FlowField,
                    BufferId::VertexBuffer,
                    AttributeId,
                    Rc::new(create_position_attribute),
                    Rc::new(|_| {}),
                    Rc::new(|_| false),
                );

                let u_texture = UniformLink::new(
                    ProgramId::PassThrough,
                    UniformId::UTexture,
                    UniformCallback::new(Rc::new(|ctx| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1i(Some(uniform_location), 0);
                    })),
                );

                let noise_texture_link =
                    TextureLink::new(TextureId::Noise, Rc::new(create_texture));

                let render_callback = RenderCallback::new(Rc::new(render));

                let mut renderer_builder = Renderer::builder();

                renderer_builder
                    .set_canvas(canvas)
                    .set_user_ctx(render_state)
                    .set_render_callback(render_callback)
                    .add_vertex_shader_src(VertexShaderId::Quad, VERTEX_SHADER.to_string())
                    .add_fragment_shader_src(
                        FragmentShaderId::FlowField,
                        FLOW_FIELD_FRAGMENT_SHADER.to_string(),
                    )
                    .add_fragment_shader_src(
                        FragmentShaderId::PassThrough,
                        PASS_THROUGH_FRAGMENT_SHADER.to_string(),
                    )
                    .add_program_link(flow_field_program_link)
                    .add_program_link(pass_through_program_link)
                    .add_buffer_link(vertex_buffer_link)
                    .add_attribute_link(a_position_link)
                    .add_uniform_link(u_texture)
                    .add_texture_link(noise_texture_link);

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
