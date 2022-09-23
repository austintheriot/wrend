#version 300 es

in vec2 a_position;

out vec2 v_tex_coord;

void main() {
  // map from (-1, 1) to (0, 1)
  v_tex_coord = a_position * 0.5 + 0.5;
  
  gl_Position = vec4(a_position, 0, 1);
}