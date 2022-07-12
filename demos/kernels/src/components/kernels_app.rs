use log::info;
use std::rc::Rc;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use webgl::{
    constants::quad::QUAD,
    renderer::{
        buffer_link::BufferLink, id::Id, id_name::IdName, program_link::ProgramLink,
        render_callback::RenderCallback, renderer::Renderer, uniform_link::UniformLink,
    },
};
use yew::{
    function_component, html, use_effect_with_deps, use_node_ref, use_state_eq, UseStateHandle,
};

const VERTEX_SHADER: &'static str = include_str!("../shaders/vertex.glsl");
const FRAGMENT_SHADER: &'static str = include_str!("../shaders/fragment.glsl");

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct ProgramId;

impl Id for ProgramId {}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UniformId {
    ExampleUniform,
}

impl Id for UniformId {}

impl Default for UniformId {
    fn default() -> Self {
        Self::ExampleUniform
    }
}

impl IdName for UniformId {
    fn name(&self) -> String {
        match self {
            UniformId::ExampleUniform => "u_example".to_string(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BufferId {
    ExampleBuffer,
}

impl Id for BufferId {}

impl Default for BufferId {
    fn default() -> Self {
        Self::ExampleBuffer
    }
}

impl IdName for BufferId {
    fn name(&self) -> String {
        match self {
            BufferId::ExampleBuffer => "a_example".to_string(),
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

#[function_component(KernelsApp)]
pub fn kernels_app() -> Html {
    let canvas_ref = use_node_ref();
    let example_state = use_state_eq(|| 0);

    use_effect_with_deps(
        {
            let canvas_ref = canvas_ref.clone();
            move |_| {
                let canvas: HtmlCanvasElement = canvas_ref
                    .cast()
                    .expect("Canvas ref should point to a canvas in the use_effect hook");

                let program_link =
                    ProgramLink::new(ProgramId, ShaderId::Vertex, ShaderId::Fragment);

                let uniform_link = UniformLink::new(
                    ProgramId,
                    UniformId::ExampleUniform,
                    Rc::new(|ctx| {
                        ctx.gl().uniform1f(Some(ctx.uniform_location()), 1.0);
                        info!("Uniform updater called!");
                    }),
                );

                let mut renderer_builder = Renderer::builder();

                let buffer_link = BufferLink::new(
                    ProgramId,
                    BufferId::ExampleBuffer,
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
                        gl.enable_vertex_attrib_array(attribute_location.into());
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
                    Rc::new(|_| {
                        info!("Renderer update callback called!");
                    }),
                    Rc::new(|_| {
                        info!("Should update buffer callback called");
                        true
                    }),
                );

                let render_callback = RenderCallback::new(Rc::new(
                    |_: &Renderer<
                        ShaderId,
                        ShaderId,
                        ProgramId,
                        UniformId,
                        BufferId,
                        UseStateHandle<i32>,
                    >| {
                        info!("Render callback was called!",);
                    },
                ));

                renderer_builder
                    .add_program_link(program_link)
                    .add_vertex_shader_src(ShaderId::Vertex, VERTEX_SHADER.to_string())
                    .add_fragment_shader_src(ShaderId::Fragment, FRAGMENT_SHADER.to_string())
                    .set_canvas(canvas)
                    .set_user_ctx(example_state)
                    .set_render_callback(render_callback)
                    .add_uniform_link(uniform_link)
                    .add_buffer_link(buffer_link);

                let renderer = renderer_builder
                    .build()
                    .expect("Renderer should successfully build");

                info!("Renderer successfully built!");

                renderer.render();

                renderer.update_uniforms();

                renderer.update_buffers();

                renderer.render();

                return || {};
            }
        },
        (),
    );

    html! {
        <>
            <p>{"This is the top-level of the Kernels portion of the app"}</p>
            <canvas ref={canvas_ref} />
        </>
    }
}
