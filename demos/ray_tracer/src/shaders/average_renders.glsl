#version 300 es

precision highp float;

// the coordinates of the texture passed in from the vertex shader
in vec2 v_tex_coord;

uniform sampler2D u_averaged_render;
uniform sampler2D u_prev_render;

out vec4 out_color;

void main() {
  vec4 _unused = texture(u_averaged_render, v_tex_coord);
  out_color = texture(u_prev_render, v_tex_coord);
}