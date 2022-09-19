use js_sys::Float32Array;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, HtmlCanvasElement, WebGl2RenderingContext};
use wrend::{
    AttributeCreateContext, AttributeLink, BufferCreateContext, BufferLink, Id, IdDefault, IdName,
    ProgramLink, Renderer, RendererData,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct ProgramId;

impl Id for ProgramId {}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct VaoId;

impl Id for VaoId {}

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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct AppState {
    count: u32,
}

const VERTEX_SHADER: &str = r#"#version 300 es
in vec2 a_position;
out vec2 v_position;
 
void main() {
    gl_Position = vec4(a_position, 0, 1);
    vec2 zero_to_two = a_position + 1.0;
    vec2 zero_to_one = zero_to_two * 0.5;
    v_position = zero_to_one;
}"#;


const FRAGMENT_SHADER: &str = r#"#version 300 es
precision mediump float;
in vec2 v_position;
out vec4 out_color;

void main() {
  out_color = vec4(v_position.x, v_position.y, v_position.x * 0.5 + v_position.y * 0.5, 1);
}"#;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let canvas: HtmlCanvasElement = window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("canvas")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    let app_state = AppState::default();

    let program_link = ProgramLink::new(ProgramId, VertexShaderId, FragmentShaderId);

    let vertex_buffer_link =
        BufferLink::new(BufferId::VertexBuffer, |ctx: &BufferCreateContext| {
            let gl = ctx.gl();
            let buffer = gl.create_buffer().unwrap();
            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
            let vertex_array = unsafe { Float32Array::view(&[-0.0, 1.0, 1.0, -1.0, -1.0, -1.0]) };
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vertex_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );

            buffer
        });

    let a_position_link = AttributeLink::new(
        VaoId,
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

    let render_callback = |renderer_data: &RendererData<
        VertexShaderId,
        FragmentShaderId,
        ProgramId,
        IdDefault,
        BufferId,
        PositionAttributeId,
        IdDefault,
        IdDefault,
        IdDefault,
        VaoId,
        AppState,
    >| {
        let gl = renderer_data.gl();
        let canvas: HtmlCanvasElement = gl.canvas().unwrap().dyn_into().unwrap();

        renderer_data.use_program(&ProgramId);
        renderer_data.use_vao(&VaoId);

        gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3);
    };

    let mut render_builder = Renderer::builder();

    render_builder
        .set_canvas(canvas)
        .set_user_ctx(app_state)
        .add_vertex_shader_src(VertexShaderId, VERTEX_SHADER.to_string())
        .add_fragment_shader_src(FragmentShaderId, FRAGMENT_SHADER.to_string())
        .add_program_link(program_link)
        .add_buffer_link(vertex_buffer_link)
        .add_attribute_link(a_position_link)
        .add_vao_link(VaoId)
        .set_render_callback(render_callback);

    let renderer = render_builder
        .build_renderer()
        .expect("Renderer should successfully build");

    renderer.render();

    Ok(())
}
