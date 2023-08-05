in vec3 in_position;
in vec3 in_normal;
in vec2 in_uv;

out vec3 vtf_normal;
out vec2 vtf_uv;

uniform mat4 xf;

void main() {
    gl_Position = xf * vec4(in_position, 1.0);
    vtf_normal = mat3(xf) * in_normal;
    vtf_uv = in_uv;
}
