#version 300 es

precision highp float;

in vec2 v_tex_coord;

uniform float u_now;

uniform sampler2D u_white_noise_texture;

out vec4 out_color;

// This logic comes from Luke Rissacher's Shadertoy demo: https://www.shadertoy.com/view/MtcGRl
// which is basically a re-discovery of flownoise
vec2 get_gradient(vec2 int_pos, float t) {
    // Texture-based rand (a bit faster on my GPU)
    float rand = texture(u_white_noise_texture, int_pos / 64.0).r;

    // Rotate gradient: random starting rotation, random rotation rate
    float angle = 6.283185 * rand + 4.0 * t * rand;
    return vec2(cos(angle), sin(angle));
}

float pseudo_3d_noise(vec3 pos) {
    vec2 i = floor(pos.xy);
    vec2 f = pos.xy - i;
    vec2 blend = f * f * (3.0 - 2.0 * f);
    float noiseVal = 
        mix(
            mix(
                dot(get_gradient(i + vec2(0, 0), pos.z), f - vec2(0, 0)),
                dot(get_gradient(i + vec2(1, 0), pos.z), f - vec2(1, 0)),
                blend.x),
            mix(
                dot(get_gradient(i + vec2(0, 1), pos.z), f - vec2(0, 1)),
                dot(get_gradient(i + vec2(1, 1), pos.z), f - vec2(1, 1)),
                blend.x),
        blend.y
    );
    return noiseVal / 0.7; // normalize to about [-1..1]
}

void main() {
    float noiseVal = 0.5 + 0.5 * pseudo_3d_noise(vec3(v_tex_coord, u_now));
    out_color = vec4(noiseVal, noiseVal, noiseVal, 1);
}