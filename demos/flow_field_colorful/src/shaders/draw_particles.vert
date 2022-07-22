#version 300 es

precision highp float;

in vec3 a_particle_position;

in vec3 a_particle_color;

out vec3 v_particle_position;

out vec3 v_particle_color;

void main() {
  gl_PointSize = 1.0;

  // transform position using matrix transformation
  gl_Position = vec4(a_particle_position, 1);

  v_particle_position = a_particle_position;

  v_particle_color = a_particle_color;
}