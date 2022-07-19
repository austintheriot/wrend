#version 300 es

precision highp float;

layout (location = 0) in vec3 a_particle_position;

void main() {
  gl_PointSize = 1.0;

  // transform position using matrix transformation
  gl_Position = vec4(a_particle_position, 1);
}