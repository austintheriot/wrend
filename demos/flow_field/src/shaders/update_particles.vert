#version 300 es

layout (location = 0) in vec3 a_particle_position;

// saved in transform feedback buffer
out vec3 o_position;

void main() {
  o_position = a_particle_position;
}