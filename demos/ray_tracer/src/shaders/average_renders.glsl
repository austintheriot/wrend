#version 300 es

precision highp float;

// the coordinates of the texture passed in from the vertex shader
in vec2 v_tex_coord;

uniform sampler2D u_averaged_render_texture;
uniform sampler2D u_prev_render_texture;
uniform int u_render_count;

out vec4 out_color;

void main() {
  vec4 prev_render = texture(u_prev_render_texture, v_tex_coord);

  if (u_render_count < 2) {
    out_color = prev_render;
  } else {
    float render_count = float(u_render_count);
    vec4 averaged_render = texture(u_averaged_render_texture, v_tex_coord);

    float averaged_render_weight = (render_count - 1.0) / render_count;
    float prev_render_weight = 1.0 / render_count;

    vec4 new_averaged_render = averaged_render * averaged_render_weight + prev_render * prev_render_weight;

    out_color = new_averaged_render;
  }
}