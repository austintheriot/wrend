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

float _get_phi(float x, float y) {
  float phi;

  if (x > 0.0) {
    phi = atan(y / x);
  } else if (x < 0.0 && y >= 0.0) {
    phi = atan(y / x) + PI;
  } else if (x < 0.0 && y < 0.0) {
    phi = atan(y / x) - PI;
  } else if (x == 0.0 && y > 0.0) {
    phi = PI / 2.0;
  } else if (x == 0.0 && y < 0.0) {
    phi = (PI / 2.0) * -1.0;
  }

  return phi;
}

float get_phi(float x, float y, float r) {
  float phi;

  if (y >= 0.0 && r != 0.0) {
    phi = acos(x / r);
  } else if (y > 0.0) {
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

  float output_color;
  if (phi > 0.6666) {
    output_color = 1.0;
  } else if (phi > 0.33333) {
    output_color = 0.5;
  } else {
    output_color = 0.0;
  }

  out_color = vec4(output_color, 0, 0, 1);
}