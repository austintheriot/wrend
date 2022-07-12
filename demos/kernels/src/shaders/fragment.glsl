#version 300 es
 
// fragment shaders don't have a default precision so we need
// to pick one. highp is a good default. It means "high precision"
precision highp float;

// the texCoords passed in from the vertex shader.
in vec2 v_position;

// we need to declare an output for the fragment shader
out vec4 out_color;

void main() {
  out_color = vec4(v_position.x, v_position.y, v_position.x * 0.5 + v_position.y * 0.5, 1);
}