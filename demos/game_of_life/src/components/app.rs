use crate::graphics::{
    buffer_id::BufferId, create_buffer::create_vertex_buffer, create_texture::create_texture,
    framebuffer_id::FramebufferId, program_id::ProgramId, render::render, shader_id::ShaderId,
    texture_id::TextureId, uniform_id::UniformId,
};
use log::info;
use std::rc::Rc;
use ui::route::Route;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use webgl::renderer::{
    animation_callback::AnimationCallback, buffer_link::BufferLink,
    framebuffer_link::FramebufferLink, program_link::ProgramLink, render_callback::RenderCallback,
    renderer::Renderer, texture_link::TextureLink, uniform_link::UniformLink,
};
use yew::{
    function_component, html, use_effect_with_deps, use_mut_ref, use_node_ref, use_state_eq,
};
use yew_router::prelude::*;

const VERTEX_SHADER: &'static str = include_str!("../shaders/vertex.glsl");
const FRAGMENT_SHADER: &'static str = include_str!("../shaders/fragment.glsl");

#[function_component(App)]
pub fn app() -> Html {
    let canvas_ref = use_node_ref();
    let example_state = use_state_eq(|| 0);
    let animation_handle = use_mut_ref(|| None);

    use_effect_with_deps(
        {
            let canvas_ref = canvas_ref.clone();
            let animation_handle = animation_handle.clone();
            move |_| {
                let canvas: HtmlCanvasElement = canvas_ref
                    .cast()
                    .expect("Canvas ref should point to a canvas in the use_effect hook");

                let program_link =
                    ProgramLink::new(ProgramId, ShaderId::Vertex, ShaderId::Fragment);

                let a_position_link = BufferLink::new(
                    ProgramId,
                    BufferId::VertexBuffer,
                    Rc::new(create_vertex_buffer),
                    Rc::new(|_| {}),
                    Rc::new(|_| false),
                );

                let u_now_link = UniformLink::new(
                    ProgramId,
                    UniformId::UNow,
                    Rc::new(|ctx| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1f(Some(uniform_location), ctx.now() as f32);
                    }),
                );

                let texture_a_link =
                    TextureLink::new(ProgramId, TextureId::A, Rc::new(create_texture));

                let framebuffer_a_link = FramebufferLink::new(
                    ProgramId,
                    FramebufferId::A,
                    Rc::new(|ctx| {
                        let texture_a = ctx.renderer_builder()
                            .texture(&TextureId::A)
                            .expect("RendererBuilder should have Texture A built when creating framebuffers");
                        let gl = ctx.gl();

                        let framebuffer_object = gl
                            .create_framebuffer()
                            .expect("WebGL2 should be able to create a WebGlFramebuffer object");
                        gl.bind_framebuffer(
                            WebGl2RenderingContext::FRAMEBUFFER,
                            Some(&framebuffer_object),
                        );
                        gl.framebuffer_texture_2d(
                            WebGl2RenderingContext::FRAMEBUFFER,
                            WebGl2RenderingContext::COLOR_ATTACHMENT0,
                            WebGl2RenderingContext::TEXTURE_2D,
                            Some(texture_a.webgl_texture()),
                            0,
                        );
                        framebuffer_object
                    }),
                );

                let render_callback = RenderCallback::new(Rc::new(render));

                let mut renderer_builder = Renderer::builder();

                renderer_builder
                    .set_canvas(canvas)
                    .set_user_ctx(example_state)
                    .set_render_callback(render_callback)
                    .add_program_link(program_link)
                    .add_vertex_shader_src(ShaderId::Vertex, VERTEX_SHADER.to_string())
                    .add_fragment_shader_src(ShaderId::Fragment, FRAGMENT_SHADER.to_string())
                    .add_buffer_link(a_position_link)
                    .add_uniform_link(u_now_link)
                    .add_texture_link(texture_a_link)
                    .add_framebuffer_link(framebuffer_a_link);

                let renderer = renderer_builder
                    .build()
                    .expect("Renderer should successfully build");

                info!("Renderer: {:#?}", renderer);

                let new_animation_handle =
                    renderer.into_animation_handle(AnimationCallback::new(Rc::new(|renderer| {
                        renderer.update_uniforms();
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
        <>
            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            <canvas ref={canvas_ref} />
        </>
    }
}
