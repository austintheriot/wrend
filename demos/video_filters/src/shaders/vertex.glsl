#version 300 es

in vec2 a_position;

out vec2 v_tex_coord;

void main() {
  // map from (-1, 1) to (0, 1)
  vec2 v_tex_coord_zero_to_one = a_position * 0.5 + 0.5;
  
  // map from (0, 1) to (1, 0) -- flipy y-axis to index textures correctly
  v_tex_coord = vec2(v_tex_coord_zero_to_one.x, 1.0 - v_tex_coord_zero_to_one.y);

  gl_Position = vec4(a_position, 0, 1);
}