use log::info;
use std::rc::Rc;
use web_sys::HtmlCanvasElement;
use webgl::renderer::{
    id::Id, id_name::IdName, program_link::ProgramLink, render_callback::RenderCallback,
    renderer::Renderer, uniform_link::UniformLink,
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
    Example
}

impl Id for UniformId {}

impl Default for UniformId {
    fn default() -> Self {
        Self::Example
    }
}

impl IdName for UniformId {
    fn name(&self) -> String {
        match self {
            UniformId::Example => "u_example".to_string(),
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
                
                let uniform_link = UniformLink::new(ProgramId, UniformId::Example, Rc::new(|ctx| {
                    ctx.gl().uniform1f(Some(ctx.uniform_location()), 1.0);
                    info!("Uniform updater called! {:?}", ctx);
                }));

                let mut renderer_builder = Renderer::builder();

                let render_callback = RenderCallback::new(Rc::new(
                    |renderer: &Renderer<
                        ShaderId,
                        ShaderId,
                        ProgramId,
                        UniformId,
                        UseStateHandle<i32>,
                    >| {
                        info!("Render callback was called! Called with {:?}", renderer);
                        if let Some(ctx) = renderer.user_ctx() {
                            let current_value = **ctx;
                            info!("Current count is {:?}", current_value);
                        }
                    },
                ));

                renderer_builder
                    .add_program_link(program_link)
                    .add_vertex_shader_src(ShaderId::Vertex, VERTEX_SHADER.to_string())
                    .add_fragment_shader_src(ShaderId::Fragment, FRAGMENT_SHADER.to_string())
                    .set_canvas(canvas)
                    .expect("Canvas should have a WebGL2RenderingContext")
                    .set_user_ctx(example_state)
                    .set_render_callback(render_callback)
                    .add_uniform_link(uniform_link);

                let renderer = renderer_builder
                    .build()
                    .expect("Renderer should successfully build");

                info!("{:?}", renderer);

                renderer.render();

                renderer.update_uniforms();

                
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
