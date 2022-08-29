use ui::route::Route;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use wrend::{
    AttributeCreateContext, AttributeLink, BufferCreateContext, BufferLink, Id, IdDefault, IdName,
    ProgramLink, Renderer, UniformContext, UniformLink, QUAD,
};
use yew::{
    function_component, html, use_effect_with_deps, use_mut_ref, use_node_ref, use_state_eq,
    UseStateHandle,
};
use yew_router::prelude::*;

const VERTEX_SHADER: &str = include_str!("../shaders/vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("../shaders/fragment.glsl");

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

#[derive(Clone, Default, Copy, PartialEq, Eq, Hash, Debug)]
pub struct VertexShaderId;

impl Id for VertexShaderId {}

#[derive(Clone, Default, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FragmentShaderId;

impl Id for FragmentShaderId {}

#[derive(Clone, Default, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PositionAttributeId;

impl Id for PositionAttributeId {}

impl IdName for PositionAttributeId {
    fn name(&self) -> String {
        String::from("a_position")
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let canvas_ref = use_node_ref();
    let example_state = use_state_eq(|| 0);
    let renderer_handle = use_mut_ref(|| None);

    use_effect_with_deps(
        {
            let canvas_ref = canvas_ref.clone();
            let renderer_handle = renderer_handle;
            move |_| {
                let canvas: HtmlCanvasElement = canvas_ref
                    .cast()
                    .expect("Canvas ref should point to a canvas in the use_effect hook");

                let program_link = ProgramLink::new(ProgramId, VertexShaderId, FragmentShaderId);

                let vertex_buffer_link = BufferLink::new(
                    BufferId::VertexBuffer,
                    |ctx: &BufferCreateContext<UseStateHandle<i32>>| {
                        let gl = ctx.gl();
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

                        buffer
                    },
                );

                let a_position_link = AttributeLink::new(
                    ProgramId,
                    BufferId::VertexBuffer,
                    PositionAttributeId,
                    |ctx: &AttributeCreateContext| {
                        let gl = ctx.gl();
                        let attribute_location = ctx.attribute_location();
                        let webgl_buffer = ctx.webgl_buffer();
                        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(webgl_buffer));
                        gl.vertex_attrib_pointer_with_i32(
                            attribute_location.into(),
                            2,
                            WebGl2RenderingContext::FLOAT,
                            false,
                            0,
                            0,
                        );
                    },
                );

                let render_callback = |renderer: &Renderer<
                    VertexShaderId,
                    FragmentShaderId,
                    ProgramId,
                    UniformId,
                    BufferId,
                    PositionAttributeId,
                    IdDefault,
                    IdDefault,
                    IdDefault,
                    ProgramId,
                    UseStateHandle<i32>,
                >| {
                    let gl = renderer.gl();
                    let canvas: HtmlCanvasElement = gl.canvas().unwrap().dyn_into().unwrap();

                    // use the appropriate program
                    renderer.use_program(&ProgramId);
                    renderer.use_vao(&ProgramId);

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
                };

                let mut renderer_builder = Renderer::builder();

                let mut u_now_link =
                    UniformLink::new(ProgramId, UniformId::UNow, |ctx: &UniformContext| {
                        let gl = ctx.gl();
                        let uniform_location = ctx.uniform_location();
                        gl.uniform1f(Some(uniform_location), ctx.now() as f32);
                    });
                u_now_link.set_use_init_callback_for_update(true);

                renderer_builder
                    .set_canvas(canvas)
                    .set_user_ctx(example_state)
                    .set_render_callback(render_callback)
                    .add_program_link(program_link)
                    .add_vertex_shader_src(VertexShaderId, VERTEX_SHADER.to_string())
                    .add_fragment_shader_src(FragmentShaderId, FRAGMENT_SHADER.to_string())
                    .add_buffer_link(vertex_buffer_link)
                    .add_attribute_link(a_position_link)
                    .add_uniform_link(u_now_link)
                    .add_vao_link(ProgramId);

                let renderer = renderer_builder
                    .build()
                    .expect("Renderer should successfully build");

                let mut new_renderer_handle = renderer.into_renderer_handle();
                new_renderer_handle.set_animation_callback(Some(
                    |renderer: &Renderer<_, _, _, _, _, _, _, _, _, _, _>| {
                        renderer.update_uniforms();
                        renderer.render();
                    },
                ));

                new_renderer_handle.start_animating();

                // save handle to keep animation going
                *renderer_handle.borrow_mut() = Some(new_renderer_handle);

                || {}
            }
        },
        (),
    );

    html! {
        <div class="hello-quad-animated">
            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            <canvas ref={canvas_ref} />
        </div>
    }
}
