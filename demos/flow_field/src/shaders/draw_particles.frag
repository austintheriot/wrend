#version 300 es

precision highp float;

uniform sampler2D u_perlin_noise_texture;

in vec3 v_particle_position;

out vec4 o_color;
 
void main() {
  vec2 uv = v_particle_position.xy * 0.5 + 0.5;
  o_color = texture(u_perlin_noise_texture, uv);
}