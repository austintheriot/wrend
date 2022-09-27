#version 300 es

precision highp float;

in vec2 v_tex_coord;

uniform sampler2D u_src_texture;

out vec4 out_color;

const float UNEVEN_TRANSPOSITION = 23.0 / 97.0;

void main() {
  vec2 split_coords = mod(v_tex_coord + UNEVEN_TRANSPOSITION, 1.0);
  out_color = texture(u_src_texture, split_coords);
}