#version 300 es

precision highp float;

// the coordinates of the texture passed in from the vertex shader
in vec2 v_tex_coord;

// our texture
uniform sampler2D u_texture;

out vec4 out_color;

const int KERNEL_SIZE = 3;

void main() {
  // the size of one pixel-- (0 --> 1) divided by the width of the texture
  vec2 one_pixel = vec2(1) / vec2(textureSize(u_texture, 0));

  // programmatically run convolution with kernel of any size
  vec4 surrounding_color_sum_vec4 = vec4(0);
  int kernel_index = 0;
  int half_of_kernel_size = KERNEL_SIZE / 2;
  for(int x = -half_of_kernel_size; x < half_of_kernel_size + 1; x++) {
    for(int y = -half_of_kernel_size; y < half_of_kernel_size + 1; y++) {
      // get texture coordinate for every element in the kernel
      vec2 texture_coord = v_tex_coord + one_pixel * vec2(x, y);
      // add color to the sume
      surrounding_color_sum_vec4 += texture(u_texture, texture_coord);
      kernel_index++;
    }
  }

  float surrounding_color_sum = surrounding_color_sum_vec4.r + surrounding_color_sum_vec4.g + surrounding_color_sum_vec4.b;

  vec4 current_color = texture(u_texture, v_tex_coord);
  float current_color_sum = current_color.r + current_color.g + current_color.b;

  // alive ones stay alive
  if (6.0 <= surrounding_color_sum && surrounding_color_sum <= 9.0) {
    out_color = vec4(current_color.rgb, 1.0);
  } 

  // dead ones come alive
  else if (current_color_sum < 0.5 && surrounding_color_sum >= 7.0) {
    out_color = vec4((current_color + surrounding_color_sum_vec4 * 0.01).rgb, 1.0);
  } 

  // alive ones die
  else {
    out_color = vec4((current_color - surrounding_color_sum_vec4 * 0.01).rgb, 1.0);
  }
}