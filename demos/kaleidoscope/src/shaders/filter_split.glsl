#version 300 es

precision highp float;

in vec2 v_tex_coord;

uniform sampler2D u_src_texture;

out vec4 out_color;

void main() {
  vec2 split_coords = mod(v_tex_coord + 0.25, 1.0);
  out_color = texture(u_src_texture, split_coords);
}