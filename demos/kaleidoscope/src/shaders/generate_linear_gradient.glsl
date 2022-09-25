#version 300 es

precision highp float;

in vec2 v_tex_coord;

out vec4 out_color;

void main() {
    out_color = vec4(v_tex_coord.x, v_tex_coord.y, v_tex_coord.x * v_tex_coord.y, 1);
}