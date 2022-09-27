#version 300 es

precision highp float;

in vec2 v_tex_coord;

uniform sampler2D u_src_texture;

out vec4 out_color;

const float PI = 3.141592653589793;
const vec2 CARTESIAN_TEXTURE_CENTER = vec2(0.27, 0.31);

// see https://en.wikipedia.org/wiki/Polar_coordinate_system for formula
float get_phi(float x, float y, float r) {
  float phi;

  if (y >= 0. && r != 0.) {
    phi = acos(x / r);
  } else if (y < 0.) {
    phi = -acos(x / r);
  }

  return phi;
}

// inputs: (r, phi), where phi ranges from (−π, π]
// output: (x, y) in cartesian coordinates
vec2 polar_to_cartesian_coordinates(vec2 polar) {
  float r = polar.x;
  float phi = polar.y;
  float x = r * cos(phi);
  float y = r * sin(phi);
  return vec2(x, y);
}

// inputs: (x, y) in cartesian coordinates
// output (r, phi), where phi ranges from (−π, π]
vec2 cartesian_to_polar_coordinates(vec2 cartesian, vec2 origin) {
  float r = distance(cartesian, origin);

  float y = cartesian.y;
  float x = cartesian.x;

  x = x - origin.x;
  y = y - origin.y;

  // prevent undefined phi
  if (x == 0.) {
    x = x + 0.00001;
  }

  // get phi (−π, π]
  float phi = get_phi(x, y, r);
  
  return vec2(r, phi);
}

void main() {
  // convert texture coordinates into polar coordinates
  vec2 polar_coords = cartesian_to_polar_coordinates(v_tex_coord, CARTESIAN_TEXTURE_CENTER);

  const float FULL_CIRCLE = 2.0 * PI;
  const float ONE_TWELTH = FULL_CIRCLE / 12.0;

  // sample only the first 1/12 of the polar coordinate plane
  polar_coords = mod(polar_coords, vec2(10000, ONE_TWELTH));

  vec2 cartesian_coords = polar_to_cartesian_coordinates(polar_coords);
  
  out_color = texture(u_src_texture, cartesian_coords);
}