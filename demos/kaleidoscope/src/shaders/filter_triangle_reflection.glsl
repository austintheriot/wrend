#version 300 es

precision highp float;

in vec2 v_tex_coord;

uniform sampler2D u_src_texture;

out vec4 out_color;

const float PI = 3.141592653589793;
const vec2 CARTESIAN_CENTER = vec2(0.5, 0.5);

float radians_to_degrees(float radians) {
  return radians * (180.0 / PI);
}

float get_phi(float x, float y, float r) {
  float phi;

  if (y >= 0.0 && r != 0.0) {
    phi = acos(x / r);
  } else if (y < 0.0) {
    phi = -acos(x / r);
  }

  return phi;
}

void main() {
  vec4 _unused = texture(u_src_texture, vec2(0, 0));

  float y = v_tex_coord.y;
  float x = v_tex_coord.x;

  // use origin
  y = y - 0.5;
  x = x - 0.5;

  if (x == 0.) {
    // prevent undefined phi
    x = x + 0.00001;
  }

  // get radius
  float r = distance(v_tex_coord, CARTESIAN_CENTER);

  // get phi (−π, π]
  float phi = get_phi(x, y, r) + _unused.x * 0.0000001;
  // map to (0, 2π]
  phi = phi + PI;
  // map to (0, 1]
  phi = phi / (2.0 * PI);

  out_color = vec4(phi, r, 0, 1);
}