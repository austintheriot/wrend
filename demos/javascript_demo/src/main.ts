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
  const attributeLocation = ctx.attributeLocation();
  const webglBuffer = ctx.webglBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, webglBuffer);
  gl.vertexAttribPointer(attributeLocation.get(), 2, gl.FLOAT, false, 0, 0);
})

const render = (renderer: Renderer) => {
  const gl = renderer.gl();
  const canvas = renderer.canvas();

  renderer.useProgram(PROGRAM_ID);
  renderer.useVAO(VAO_ID);

  gl.viewport(0, 0, canvas.width, canvas.height);
  gl.clearColor(0, 0, 0, 0);
  gl.clear(gl.COLOR_BUFFER_BIT);
  gl.drawArrays(gl.TRIANGLES, 0, 6);
};

const renderer: Renderer = Renderer.builder()
  .setCanvas(canvas)
  .setRenderCallback(() => render(renderer))
  .addProgramLink(programLink)
  .addVertexShaderSrc(VERTEX_SHADER_ID, vertexShader)
  .addFragmentShaderSrc(FRAGMENT_SHADER_ID, fragmentShader)
  .addBufferLink(vertexBufferLink)
  .addAttributeLink(aPositionLink)
  .addVAOLink(VAO_ID)
  .build();

renderer.render();