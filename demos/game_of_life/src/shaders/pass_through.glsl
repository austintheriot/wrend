#version 300 es

precision highp float;

// the coordinates of the texture passed in from the vertex shader
in vec2 v_tex_coord;

// our texture
uniform sampler2D u_texture;

out vec4 out_color;

void main() {
  out_color = texture(u_texture, v_tex_coord);
}