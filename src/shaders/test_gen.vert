layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec2 in_uv;

uniform mat4 uni_xf;
uniform vec4 frag_vec;
uniform int int_uni;

out vec3 vtf_normal;
out vec2 vtf_uv;

void main() {
    gl_Position = uni_xf * vec4(in_position, 1.0) + frag_vec + vec4(int_uni);
    vtf_normal = in_normal;
    vtf_uv = in_uv;
}
