use std::rc::Rc;
use ui::route::Route;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use webgl::{
    constants::quad::QUAD,
    renderer::{
        animation_callback::AnimationCallback, buffer_link::BufferLink, id::Id, id_name::IdName,
        program_link::ProgramLink, render_callback::RenderCallback, renderer::Renderer,
        uniform_link::UniformLink, default_id::DefaultId,
    },
};
use yew::{
    function_component, html, use_effect_with_deps, use_mut_ref, use_node_ref, use_state_eq,
    UseStateHandle,
};
use yew_router::prelude::*;

const VERTEX_SHADER: &'static str = include_str!("../shaders/vertex.glsl");
const FRAGMENT_SHADER: &'static str = include_str!("../shaders/fragment.glsl");

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct ProgramId;

impl Id for ProgramId {}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UniformId {
    UNow,
}

impl Id for UniformId {}

impl Default for UniformId {
    fn default() -> Self {
        Self::UNow
    }
}

impl IdName for UniformId {
    fn name(&self) -> String {
        match self {
            UniformId::UNow => "u_now".to_string(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BufferId {
    VertexBuffer,
}

impl Id for BufferId {}

impl Default for BufferId {
    fn default() -> Self {
        Self::VertexBuffer
    }
}

impl IdName for BufferId {
    fn name(&self) -> String {
        match self {
            BufferId::VertexBuffer => "a_position".to_string(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ShaderId {
    Vertex,
    Fragment,
}

impl Id for ShaderId {}

impl Default for ShaderId {
    fn default() -> Self {
        Self::Vertex
    }
}

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
                    Rc::new(|ctx| {
                        let gl = ctx.gl();
                        let attribute_location = ctx.attribute_location();

                        let buffer = gl.create_buffer().unwrap();
                        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

                        // requires `unsafe` since we're creating a raw view into wasm memory,
                        // but this array is static, so it shouldn't cause any issues
                        let vertex_array = unsafe { js_sys::Float32Array::view(&QUAD) };
                        gl.buffer_data_with_array_buffer_view(
                            WebGl2RenderingContext::ARRAY_BUFFER,
                            &vertex_array,
                            WebGl2RenderingContext::STATIC_DRAW,
                        );
                        gl.vertex_attrib_pointer_with_i32(
                            attribute_location.into(),
                            2,
                            WebGl2RenderingContext::FLOAT,
                            false,
                            0,
                            0,
                        );

                        buffer
                    }),
                    Rc::new(|_| {}),
                    Rc::new(|_| false),
                );

                let render_callback = RenderCallback::new(Rc::new(
                    |renderer: &Renderer<
                        ShaderId,
                        ShaderId,
                        ProgramId,
                        UniformId,
                        BufferId,
                        DefaultId,
                        DefaultId,
                        UseStateHandle<i32>,
                    >| {
                        let gl = renderer.gl();
                        let canvas: HtmlCanvasElement = gl.canvas().unwrap().dyn_into().unwrap();

                        // use the appropriate program
                        gl.use_program(renderer.programs().get(&ProgramId));

                        // sync canvas dimensions with viewport
                        gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

                        // clear canvas
                        gl.clear_color(0.0, 0.0, 0.0, 0.0);
                        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

                        // draw
                        let primitive_type = WebGl2RenderingContext::TRIANGLES; // draws a triangle after shader is run every 3 times
                        let offset = 0;
                        let count = 6; // this will execute vertex shader 3 times
                        gl.draw_arrays(primitive_type, offset, count);
                    },
                ));

                let mut renderer_builder = Renderer::builder();

                let u_now_link = UniformLink::new(
                    ProgramId,
                    UniformId::UNow,
                    Rc::new(|ctx| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1f(Some(uniform_location), ctx.now() as f32);
                    }),
                );

                renderer_builder
                    .set_canvas(canvas)
                    .set_user_ctx(example_state)
                    .set_render_callback(render_callback)
                    .add_program_link(program_link)
                    .add_vertex_shader_src(ShaderId::Vertex, VERTEX_SHADER.to_string())
                    .add_fragment_shader_src(ShaderId::Fragment, FRAGMENT_SHADER.to_string())
                    .add_buffer_link(a_position_link)
                    .add_uniform_link(u_now_link);

                let renderer = renderer_builder
                    .build()
                    .expect("Renderer should successfully build");

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
