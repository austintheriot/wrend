#version 300 es

uniform float u_example;
 
void main() {
    gl_Position = vec4(u_example, 0.0, 0.0, 1.0);
  }