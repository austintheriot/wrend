use std::rc::Rc;

use log::info;
use web_sys::HtmlCanvasElement;
use webgl::renderer::{
    program_link::ProgramLink,
    render_callback::{self, RenderCallback},
    renderer::Renderer,
};
use yew::{function_component, html, use_effect_with_deps, use_node_ref};

const VERTEX_SHADER: &'static str = include_str!("../shaders/vertex.glsl");
const FRAGMENT_SHADER: &'static str = include_str!("../shaders/fragment.glsl");

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ShaderId {
    Vertex,
    Fragment,
}

impl Default for ShaderId {
    fn default() -> Self {
        Self::Vertex
    }
}

#[function_component(KernelsApp)]
pub fn kernels_app() -> Html {
    let canvas_ref = use_node_ref();

    use_effect_with_deps(
        {
            let canvas_ref = canvas_ref.clone();
            move |_| {
                let canvas: HtmlCanvasElement = canvas_ref
                    .cast()
                    .expect("Canvas ref should point to a canvas in the use_effect hook");

                let program_link = ProgramLink::new(ShaderId::Vertex, ShaderId::Fragment);

                let mut renderer_builder = Renderer::builder();

                let render_callback = RenderCallback::new(Rc::new(|renderer| {
                    info!("Render callback was called! Called with {:?}", renderer);
                }));

                renderer_builder
                    .add_program_link(program_link)
                    .add_vertex_shader_src(ShaderId::Vertex, VERTEX_SHADER.to_string())
                    .add_fragment_shader_src(ShaderId::Fragment, FRAGMENT_SHADER.to_string())
                    .set_canvas(canvas)
                    .expect("Canvas should have a WebGL2RenderingContext")
                    .set_render_callback(render_callback);

                let renderer = renderer_builder.build().expect("Renderer should successfully build");

                info!("{:?}", renderer);

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
