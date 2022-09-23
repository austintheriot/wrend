#version 300 es

precision highp float;

in vec2 v_tex_coord;

out vec4 out_color;

const vec2 TEXTURE_CENTER = vec2(0.5, 0.5);

void main() {
    float distance_from_center = distance(v_tex_coord, TEXTURE_CENTER);
    out_color = vec4(distance_from_center, distance_from_center, distance_from_center, 1);
}