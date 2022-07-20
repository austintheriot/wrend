#version 300 es

precision highp float;

// the coordinates of the texture passed in from the vertex shader
in vec2 v_tex_coord;

uniform sampler2D u_perlin_noise_texture;

out vec4 out_color;

void main() {
  vec2 uv = v_tex_coord * 0.5 + 0.5;
  vec4 _unused = vec4(texture(u_perlin_noise_texture, uv).rgb, 1);
  out_color = vec4(1, 0, 0, 1);
}