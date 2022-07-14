#version 300 es

precision highp float;

// the coordinates of the texture passed in from the vertex shader
in vec2 v_tex_coord;

// our texture
uniform sampler2D u_texture;

out vec4 out_color;

const int KERNEL_SIZE = 11;

void main() {
  // the size of one pixel-- (0 --> 1) divided by the width of the texture
  vec2 one_pixel = vec2(1) / vec2(textureSize(u_texture, 0));

  // programmatically run convolution with kernel of any size
  float sum = 0.0;
  int kernel_index = 0;
  int half_of_kernel_size = KERNEL_SIZE / 2;
  for(int x = -half_of_kernel_size; x < half_of_kernel_size + 1; x++) {
    for(int y = -half_of_kernel_size; y < half_of_kernel_size + 1; y++) {
      // get texture coordinate for every element in the kernel
      vec2 texture_coord = v_tex_coord + one_pixel * vec2(x, y);
      // add color to the sume
      sum += texture(u_texture, texture_coord).r;
      kernel_index++;
    }
  }

  if (0.0 <= sum && sum <= 60.0) {
    out_color = vec4(0.0, 0.0, 0.0, 1.0);
  }
  else {
    out_color = vec4(1.0, 1.0, 1.0, 1.0);
  }
}