#version 300 es

const int SAMPLE_OFFSET = 0;

precision highp float;

uniform sampler2D u_src_video_texture;

uniform float u_kernel[9];
const int KERNEL_SIZE = int(sqrt(float(u_kernel.length())));

in vec2 v_tex_coord;
out vec4 out_color;

vec4 get_offset_px(float x, float y) {
      // the size of one pixel (0 --> 1) divided by the width of the texture
    vec2 one_pixel = vec2(1) / vec2(textureSize(u_src_video_texture, 0));
    vec2 coord = v_tex_coord + one_pixel * vec2(x, y);
    vec4 px = texture(u_src_video_texture, coord);
    return px;
}

vec4 get_convolution_sum(int offset_x, int offset_y) {
    vec4 sum = vec4(0.);
    int kernel_index = 0;
    int half_of_kernel_size = KERNEL_SIZE / 2;
    for(int x = -half_of_kernel_size; x < half_of_kernel_size + 1; x++) {
        for(int y = -half_of_kernel_size; y < half_of_kernel_size + 1; y++) {
            vec4 src_px_value = get_offset_px(float(x + offset_x), float(y + offset_y));
            sum += src_px_value * u_kernel[kernel_index];
            kernel_index++;
        }
    }
    return sum;
}

void main() {
    float kernel_weight = 0.0;
    for (int i = 0; i < KERNEL_SIZE; i++) {
        kernel_weight += u_kernel[i];
    } 

    vec4 color_sum = get_convolution_sum(SAMPLE_OFFSET, SAMPLE_OFFSET);
    out_color = vec4((color_sum / kernel_weight).rgb, 1);
}