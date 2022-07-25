#version 300 es

precision highp float;

// the coordinates of the texture passed in from the vertex shader
in vec2 v_tex_coord;

uniform sampler2D u_averaged_render_texture_a;
uniform sampler2D u_averaged_render_texture_b;
uniform sampler2D u_prev_render_texture;

out vec4 out_color;

void main() {
  vec4 _unused_a = texture(u_averaged_render_texture_a, v_tex_coord);
  vec4 _unused_b = texture(u_averaged_render_texture_b, v_tex_coord);
  out_color = texture(u_prev_render_texture, v_tex_coord);
}