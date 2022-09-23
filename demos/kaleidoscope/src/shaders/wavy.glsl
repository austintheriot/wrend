#version 300 es

precision highp float;

in vec2 v_tex_coord;

uniform sampler2D u_src_video_texture;

uniform float u_now;

out vec4 out_color;

void main() {
    float y_offset = sin(u_now + v_tex_coord.x * 10.0) / 10.0;
    vec2 tex_coord_offset = vec2(v_tex_coord.x, v_tex_coord.y + y_offset);
    out_color = texture(u_src_video_texture, tex_coord_offset);
}