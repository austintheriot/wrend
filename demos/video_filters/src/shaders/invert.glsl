#version 300 es

precision highp float;

in vec2 v_tex_coord;

uniform sampler2D u_src_video_texture;

out vec4 out_color;

void main() {
  vec4 original_texel_value = texture(u_src_video_texture, v_tex_coord);
  out_color = vec4(1.0 - original_texel_value.rgb, 1.0);
}