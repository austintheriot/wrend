#version 300 es

const float TWO_PI = 6.283185307179586;

const float MOVEMENT_DAMPENING = 0.001;

layout (location = 0) in vec3 a_particle_position;

uniform sampler2D u_perlin_noise_texture;

// saved in transform feedback buffer
out vec3 o_position;

void main() {
  // vec2 uv = a_particle_position.xy * 0.5 + 0.5;
  // uv = clamp(uv, vec2(0.0), vec2(1.0));
  // vec2 perlin_value = texture(u_perlin_noise_texture, uv).rg;
  // perlin_value = clamp(perlin_value, vec2(0.0), vec2(1.0));
  // vec2 movement = vec2(cos(perlin_value.x * TWO_PI), sin(perlin_value.y * TWO_PI));
  // vec2 movement_slowed = movement * MOVEMENT_DAMPENING;
  // vec3 new_location = vec3(a_particle_position.xy + movement_slowed, 0.0);
  vec4 _unused = texture(u_perlin_noise_texture, vec2(0.0, 0.0));
  o_position = a_particle_position + _unused.rgb * 0.0;
}