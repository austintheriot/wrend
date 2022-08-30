import './style.css'
import { AttributeLink, BufferLink, ProgramLink, Renderer, enableErrorMessages } from 'wrend';
import vertexShader from './shaders/vertex.glsl?raw';
import fragmentShader from './shaders/fragment.glsl?raw';

enableErrorMessages();

const canvas = document.querySelector('canvas') as HTMLCanvasElement;

const QUAD = [
  -1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0,
] as const;

const PROGRAM_ID = 'program';
const VAO_ID = 'vao';
const VERTEX_SHADER_ID = 'vertex_shader';
const FRAGMENT_SHADER_ID = 'fragment_shader';
const VERTEX_BUFFER_ID = 'buffer_id';
const POSITION_ATTRIBUTE_ID = 'a_position';

const programLink = new ProgramLink(PROGRAM_ID, VERTEX_SHADER_ID, FRAGMENT_SHADER_ID);

const vertexBufferLink = new BufferLink(VERTEX_BUFFER_ID, (ctx) => {
  const gl = ctx.gl();
  const buffer = gl.createBuffer() as WebGLBuffer;
  gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(QUAD), gl.STATIC_DRAW);
  return buffer;
})

const aPositionLink = new AttributeLink([VAO_ID], VERTEX_BUFFER_ID, POSITION_ATTRIBUTE_ID, (ctx) => {
  const gl = ctx.gl();
  const attributeLocation = ctx.attribute_location();
  const webglBuffer = ctx.webgl_buffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, webglBuffer);
  gl.vertexAttribPointer(attributeLocation.get(), 2, gl.FLOAT, false, 0, 0);
});

const renderer = Renderer.builder()
  .set_canvas(canvas)
  .set_render_callback(() => {
    const gl = renderer.gl();
    const canvas = renderer.canvas();

    renderer.use_program(PROGRAM_ID);
    renderer.use_vao(VAO_ID);

    gl.viewport(0, 0, canvas.width, canvas.height);
    gl.clearColor(0, 0, 0, 0);
    gl.clear(gl.COLOR_BUFFER_BIT);
    gl.drawArrays(gl.TRIANGLES, 0, 6);
  })
  .add_program_link(programLink)
  .add_vertex_shader_src(VERTEX_SHADER_ID, vertexShader)
  .add_fragment_shader_src(FRAGMENT_SHADER_ID, fragmentShader)
  .add_buffer_link(vertexBufferLink)
  .add_attribute_link(aPositionLink)
  .add_vao_link(VAO_ID)
  .build();

renderer.render();