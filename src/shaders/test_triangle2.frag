layout(location = 0) out vec4 frag_color;

in vec3 vtf_xyz;

void main() {
    frag_color = vec4(vtf_xyz, 1.0);
}
