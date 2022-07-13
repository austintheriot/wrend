#version 300 es
 
precision highp float;

in vec2 v_position;

uniform float u_now;

// we need to declare an output for the fragment shader
out vec4 out_color;

void main() {
  float red = (sin(u_now) + 1.0) * 0.5;
  out_color = vec4(red, v_position.x, v_position.y, 1.0);
}