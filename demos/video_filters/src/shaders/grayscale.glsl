#version 300 es

precision highp float;

uniform sampler2D u_src_video_texture;

in vec2 v_tex_coord;

out vec4 out_color;

const vec3 GRAYSCALE_WEIGHTS = vec3(0.2126, 0.7152, 0.0722);

float get_grayscale_average(vec4 px) {
    vec3 weighted_vec = px.rgb * GRAYSCALE_WEIGHTS;
    return weighted_vec.r + weighted_vec.g + weighted_vec.b;
}

void main() {
    vec4 source_pixel_value = texture(u_src_video_texture, v_tex_coord);
    float average = get_grayscale_average(source_pixel_value);
    out_color = vec4(average, average, average, 1.0);
}