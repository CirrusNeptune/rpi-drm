in vec3 in_position;
in vec3 in_tangent;
in vec3 in_bitangent;
in vec3 in_normal;
in vec2 in_uv;

out vec3 vtf_light_dir_tangentspace;
out vec3 vtf_eye_dir_tangentspace;
out vec2 vtf_uv;

uniform mat4 xf;
uniform mat3 xfn;

void main() {
    gl_Position = xf * vec4(in_position, 1.0);
    vec3 tangent_cameraspace = xfn * in_tangent;
    vec3 bitangent_cameraspace = xfn * in_bitangent;
    vec3 normal_cameraspace = xfn * in_normal;
    mat3 TBN = transpose(mat3(
        tangent_cameraspace,
        bitangent_cameraspace,
        normal_cameraspace
    ));
    vec3 light_dir_cameraspace = normalize(vec3(1,1,1));
    vec3 eye_dir_cameraspace = vec3(0,0,-1);
    vtf_light_dir_tangentspace = TBN * light_dir_cameraspace;
    vtf_eye_dir_tangentspace = TBN * eye_dir_cameraspace;
    vtf_uv = in_uv;
}
