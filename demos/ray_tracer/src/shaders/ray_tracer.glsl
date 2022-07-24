#version 300 es
precision mediump float;

#define PI 3.141592653589793
#define MAX_T 1e5
#define MIN_T 0.001

// PSEUDO-RANDOM NUMBER GENERATORS //////////////////////////////////////////////////////
// global seed is initialized in main, and then each hash function alters 
// the global seed, increasing the stochasticity of the system
float global_seed = 0.;

// Hash functions by Nimitz:
// https://www.shadertoy.com/view/Xt3cDn
uint base_hash(uvec2 p) {
  p = 1103515245U * ((p >> 1U) ^ (p.yx));
  uint h32 = 1103515245U * ((p.x) ^ (p.y >> 3U));
  return h32 ^ (h32 >> 16);
}

float hash1(inout float seed) {
    uint n = base_hash(floatBitsToUint(vec2(seed += .1, seed += .1)));
    return float(n)*(1.0/float(0xffffffffU));
}

vec2 hash2(inout float seed) {
  uint n = base_hash(floatBitsToUint(vec2(seed += .1, seed += .1)));
  uvec2 rz = uvec2(n, n * 48271U);
  return vec2(rz.xy & uvec2(0x7fffffffU)) / float(0x7fffffff);
}

vec3 hash3(inout float seed) {
  uint n = base_hash(floatBitsToUint(vec2(seed += .1, seed += .1)));
  uvec3 rz = uvec3(n, n * 16807U, n * 48271U);
  return vec3(rz & uvec3(0x7fffffffU)) / float(0x7fffffff);
}

// STRUCTS //////////////////////////////////////////////////////
struct Ray {
  vec3 origin;
  vec3 direction;
};

// Material types
#define DIFFUSE 0
#define METAL 1
#define GLASS 2
struct Material {
  int type;
  vec3 albedo; // or "reflectance"
  float fuzz; // used for duller metals
  float refraction_index; // used for glass
};

struct Sphere {
  vec3 center;
  float radius;
  Material material;
  int is_active;
  int uuid;
};

struct HitRecord {
  vec3 hit_point;
  float hit_t;
  vec3 normal;
  bool front_face;
  Material material;
  int uuid;
};


// INPUTS / OUTPUTS //////////////////////////////////////////////////////
in vec2 v_position;

out vec4 o_color;

// video frame, received as a 2d texture
uniform sampler2D u_texture;
uniform float u_width;
uniform float u_height;
uniform float u_time;
uniform int u_samples_per_pixel;
uniform float u_aspect_ratio;
uniform float u_viewport_height;
uniform float u_viewport_width;
uniform float u_focal_length;
uniform vec3 u_camera_origin;
uniform vec3 u_horizontal;
uniform vec3 u_vertical;
uniform vec3 u_lower_left_corner;
uniform int u_max_depth;
uniform int u_render_count;
uniform bool u_should_average;
uniform float u_last_frame_weight;
uniform float u_lens_radius;
uniform vec3 u_u;
uniform vec3 u_v;
uniform vec3 u_w;
uniform int u_enable_debugging;
uniform int u_selected_object;
uniform vec3 u_cursor_point;
uniform Sphere[15] u_sphere_list;

// FUNCTIONS //////////////////////////////////////////////////////
vec3 ray_at(in Ray r, float hit_t) {
  return r.origin + r.direction * hit_t;
}

float length_squared(in vec3 v) {
  return pow(length(v), 2.);
}

vec3 random_in_unit_sphere() {
  // no idea how this algorithm works, but it works much better than the one I was using.
  // From reinder https://www.shadertoy.com/view/llVcDz
  vec3 h = hash3(global_seed) * vec3(2., PI * 2., 1.) - vec3(1., 0., 0.);
  float phi = h.y;
  float r = pow(h.z, 1. / 3.);
  return r * vec3(sqrt(1. - h.x * h.x) * vec2(sin(phi), cos(phi)), h.x);
}

vec2 random_in_unit_circle() {
  float a = hash1(global_seed) * 2. * PI;
  float r = sqrt(hash1(global_seed));
  float x = r * cos(a);
  float y = r * sin(a);
  return vec2(x, y);
}

vec3 random_unit_vec() {
  return normalize(random_in_unit_sphere());
}

// records whether a hit happened to the front or back face of an object
void set_hit_record_front_face(inout HitRecord hit_record, in Ray r, in vec3 outward_normal) {
  hit_record.front_face = dot(r.direction, outward_normal) < 0.;
  if (hit_record.front_face) {
    hit_record.normal = outward_normal;
  } else {
    hit_record.normal = -outward_normal;
  }
}

bool hit_sphere(in Sphere sphere, in Ray r, in float t_min, in float t_max, inout HitRecord hit_record) {
  vec3 oc = r.origin - sphere.center;
  float a = length_squared(r.direction);
  float half_b = dot(oc, r.direction);
  float c = length_squared(oc) - pow(sphere.radius, 2.);
  float discriminant = pow(half_b, 2.) - a * c;

  // no hit
  if (discriminant < 0.)
    return false;

  // there was a hit, but it's not within an acceptable range
  float sqrtd = sqrt(discriminant);
  float root = (-half_b - sqrtd) / a;
  if (root < t_min || t_max < root) {
    root = (-half_b + sqrtd) / a;
    if (root < t_min || t_max < root) {
      return false;
    }
  }

  hit_record.material = sphere.material;
  hit_record.hit_t = root;
  hit_record.hit_point = ray_at(r, hit_record.hit_t);
  hit_record.uuid = sphere.uuid;
  vec3 outward_normal = (hit_record.hit_point - sphere.center) / sphere.radius;
  set_hit_record_front_face(hit_record, r, outward_normal);
  return true;
}

bool hit_world(in Ray r, in float t_min, in float t_max, inout HitRecord hit_record) {
  // test whether any geometry was hit. If it was, the hit_record will be updated with
  // the new hit data if the new hit was closer to the camera than the previous hit
  bool hit_anything = false;
  float closest_so_far = t_max;
  HitRecord temp_hit_record;

  for(int i = 0; i < u_sphere_list.length(); i++) {
    Sphere sphere = u_sphere_list[i];
    if (sphere.is_active == 0) {
      break;
    }

    if (hit_sphere(sphere, r, t_min, closest_so_far, temp_hit_record)) {
      hit_anything = true;
      closest_so_far = temp_hit_record.hit_t;
      hit_record = temp_hit_record;
    }
  } 

  return hit_anything;
}

bool near_zero(in vec3 v) {
  float low_extreme = 0.00001;
  return (v.x < low_extreme) && (v.y < low_extreme) && (v.z < low_extreme);
}

// Schlick's approximation for reflectance
float reflectance(in float cosine, in float reflection_index) {
  float r0 = pow((1. - reflection_index) / (1. + reflection_index), 2.);
  return r0 + (1. - r0) * pow((1. - cosine), 5.);
}

// scatters a ray depending on what material was intersected with
bool scatter(in Ray r, in HitRecord hit_record, out vec3 attenuation, out Ray scattered_ray) {
  // DIFFUSE
  if (hit_record.material.type == DIFFUSE) {
    // color attenuation on reflection
    attenuation = hit_record.material.albedo;

    // shoot ray off in random direction again
    vec3 scatter_direction = hit_record.normal + random_unit_vec();

    // ignore when scattered direction becomes close to 0 for now
    // scatter direction can become close to 0 if opposite the normal vector 
    // (which can cause infinities later on)
    // if (near_zero(scatter_direction)) {
    //   scatter_direction = normalize(hit_record.normal + random_unit_vec() * 0.2);
    // }

    scattered_ray = Ray(hit_record.hit_point, scatter_direction);

    return true;
  } 

  // METAL
  if (hit_record.material.type == METAL) {
    // color attenuation on reflection
    attenuation = hit_record.material.albedo;

    // reflect ray off the surface
    vec3 reflected_direction = reflect(r.direction, hit_record.normal);

    // add in "fuzz" (optional)
    vec3 fuzzed_direction = reflected_direction + hit_record.material.fuzz * random_in_unit_sphere();
    scattered_ray = Ray(hit_record.hit_point, fuzzed_direction);

    // count any rays that are reflected below the surface as  "absorbed"
    bool reflected_above_surface = dot(hit_record.normal, fuzzed_direction) > 0.;

    return reflected_above_surface;
  } 

  // GLASS
  if (hit_record.material.type == GLASS) {
    // color attenuation on reflection
    attenuation = hit_record.material.albedo;

    // refraction differs when colliding from the front or back face
    float refraction_ratio = hit_record.front_face ? (1.0 / hit_record.material.refraction_index) : hit_record.material.refraction_index;

    vec3 unit_direction = normalize(r.direction);
    float cos_theta = min(dot(-unit_direction, hit_record.normal), 1.0);
    float sin_theta = sqrt(1.0 - cos_theta * cos_theta);

    // cannot refract when there is no real solution to Snell's law
    bool cannot_refract = refraction_ratio * sin_theta > 1.0;

    // there is a random chance of the ray reflecting
    // --chance increases as reflectance approximation increases
    float reflectance_amount = reflectance(cos_theta, refraction_ratio);
    float random_float = hash1(global_seed);

    // when the ray cannot refract (or when it's reflectance 
    // approximation is high), it reflects instead
    vec3 direction;
    if (cannot_refract || reflectance_amount > random_float) {
      direction = reflect(unit_direction, hit_record.normal);
    } else {
      direction = refract(unit_direction, hit_record.normal, refraction_ratio);
    }

    scattered_ray = Ray(hit_record.hit_point, direction);

    // never absorbs light
    return true;
  } 

// unrecognized material integer (likely an error)
  return false;
}

// default background color when no intersection color was found
vec3 background(in Ray r) {
  vec3 unit_direction = normalize(r.direction);
  float t = 0.5 * (unit_direction.y + 1.0);
  vec3 gradient = mix(vec3(1.0, 1.0, 1.0), vec3(0.5, 0.7, 1.0), t);
  return gradient;
}

// determine the color that a ray should be
vec3 ray_color(in Ray r) {
  vec3 color = vec3(1.);

  for(int i = 0; i < u_max_depth; i++) {
    // test for collisions with any geometry
    // hit record gets modified with hit details if there was a hit
    HitRecord hit_record;
    if (hit_world(r, MIN_T, MAX_T, hit_record)) {

      // color using debugging tools
      if (u_enable_debugging != 0) {
        // highlight where cursor is intersecting world
        if (length(hit_record.hit_point - u_cursor_point) < 0.1) {
          return vec3(0., 0., 1.);
        }

        // highlight outline of the object that the cursor is hovering on
        bool is_hit_on_outline = dot(hit_record.normal, r.direction) > -0.05;
        if (hit_record.uuid == u_selected_object && is_hit_on_outline) {
          return vec3(1., 0., 0.);
        }
      }

      // color using normal ray calculations
      vec3 attenuation;
      Ray scattered_ray;
      bool did_scatter = scatter(r, hit_record, attenuation, scattered_ray);
      if (did_scatter) {
        r = scattered_ray;
        color *= attenuation;
      } else {
        return vec3(0.);
      }

    } else {
        // no hit, return the sky gradient background
      vec3 background_gradient = background(r);
      return color * background_gradient;
    }
  }

  return color;
}

// create ray from camera origin to viewport
Ray get_ray_from_camera(in vec2 st) {
  // adding a camera lens offset allows simulating a depth of field effect
  vec2 random_point_on_camera_lens = u_lens_radius * random_in_unit_circle();
  vec3 viewport_offset = u_u * random_point_on_camera_lens.x + u_v * random_point_on_camera_lens.y;

  // direction from camera origin to the viewport
  vec3 ray_direction = u_lower_left_corner + st.s * u_horizontal + st.t * u_vertical - u_camera_origin - viewport_offset;

  return Ray(u_camera_origin + viewport_offset, ray_direction);
}

// set up global seed for simmulated randomness
void init_global_seed() {
  // I got this seed initialization from reinder https://www.shadertoy.com/view/llVcDz
  global_seed = float(base_hash(floatBitsToUint(v_position))) / float(0xffffffffU) + u_time;
}

// accumulates color from each ray and averages them out
vec3 get_pixel_color(in vec2 st) {
  // accumulate color per pixel
  vec3 color = vec3(0.);

  for(int i = 0; i < u_samples_per_pixel; i++) {
    vec2 random = hash2(global_seed);
    vec2 random_within_pixel = random / vec2(u_width, u_height);

    // pixel coordinate +/- the value of 1 pixel
    vec2 randomized_st = st + random_within_pixel;
    Ray r = get_ray_from_camera(randomized_st);

    color += ray_color(r);
  }

  // scale color by number of samples
  float scale = (1. / float(u_samples_per_pixel));
  color *= scale;

  // gamma correction
  color = sqrt(color);

  return color;
}

// either do a plain render or average this frame with 
// the previous one, depending on global settings
void render(in vec3 pixel_color, in vec2 st) {
  vec4 prev_frame = texture(u_texture, st);
  float render_count = float(u_render_count);
  if (u_should_average) {
    if (prev_frame.a == 0. || u_render_count <= 1) {
      // not enough data to average, render it straight
      o_color = vec4(pixel_color, 1.);
    } else {
      // average this frame with previous frames
      float total_frames = render_count + u_last_frame_weight;
      vec3 merged_color = (prev_frame.rgb * render_count + pixel_color * u_last_frame_weight) / total_frames;
      o_color = vec4(merged_color, 1.);
    }
  } else {
    // do a plain rendering (no averaging)
    o_color = vec4(pixel_color, 1.);
  }
}

void main() {
  init_global_seed();

  // current position on viewport, mapped from -1->1 to 0->1
  vec2 st = (v_position + 1.) * 0.5;
  vec3 pixel_color = get_pixel_color(st);
  render(pixel_color, st);
}