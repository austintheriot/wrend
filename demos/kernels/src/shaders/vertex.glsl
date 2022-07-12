#version 300 es

uniform float u_example;

in vec2 a_example;
 
void main() {
    gl_Position = vec4(a_example, u_example, 1.0);
  }