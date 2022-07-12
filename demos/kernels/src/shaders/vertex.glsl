#version 300 es

in vec2 a_position;
 
out vec2 v_position;
 
void main() {
    gl_Position = vec4(a_position, 0, 1);

    // map to (0. -> 2.)
    vec2 zero_to_two = a_position + 1.0;
    // map to (0. -> 1.)
    vec2 zero_to_one = zero_to_two * 0.5;

    // pass texture coordinates on to the fragment shader
    v_position = zero_to_one;
  }