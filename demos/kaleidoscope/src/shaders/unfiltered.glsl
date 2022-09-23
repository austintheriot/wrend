#version 300 es

precision highp float;

in vec2 v_tex_coord;

uniform sampler2D u_src_video_texture;

out vec4 out_color;

void main() {
  out_color = texture(u_src_video_texture, v_tex_coord);
}