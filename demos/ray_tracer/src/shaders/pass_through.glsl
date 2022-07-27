#version 300 es

precision highp float;

// the coordinates of the texture passed in from the vertex shader
in vec2 v_tex_coord;

uniform sampler2D u_averaged_render_texture;
uniform sampler2D u_prev_render_texture;
uniform int u_render_count;

out vec4 out_color;

void main() {
  if (u_render_count < 2) {
    out_color = texture(u_prev_render_texture, v_tex_coord);
  } else {
    out_color = texture(u_averaged_render_texture, v_tex_coord);
  }
}