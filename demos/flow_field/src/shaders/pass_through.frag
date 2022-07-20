#version 300 es

precision highp float;

// the coordinates of the texture passed in from the vertex shader
in vec2 v_tex_coord;

uniform sampler2D u_prev_frame_texture_a;
uniform sampler2D u_prev_frame_texture_b;

out vec4 out_color;

void main() {
  vec2 uv = v_tex_coord * 0.5 + 0.5;

  vec4 prev_frame_a_color = texture(u_prev_frame_texture_a, uv);
  vec4 prev_frame_b_color = texture(u_prev_frame_texture_b, uv);

  out_color = vec4(prev_frame_a_color.rgb, 1);
}