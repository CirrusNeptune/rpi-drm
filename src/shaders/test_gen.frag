in vec3 vtf_normal;
in vec2 vtf_uv;

layout(location = 0) out vec4 out_color;

uniform vec4 frag_vec;
uniform vec4 frag_vec2;

void main() {
    out_color = vec4(vtf_normal + vec3(vtf_uv, 0.0), 1.0) + frag_vec + frag_vec2;
}
