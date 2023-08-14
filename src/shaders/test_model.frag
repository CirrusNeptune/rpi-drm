in vec3 vtf_light_dir_tangentspace;
in vec3 vtf_eye_dir_tangentspace;
in vec2 vtf_uv;

uniform sampler2D tex;

layout(location = 0) out vec4 frag_color;

void main() {
    vec3 normal_tangentspace = texture(tex, vtf_uv).rgb * 2 - 1;
    float diffuse = dot(normal_tangentspace, vtf_light_dir_tangentspace);
    float specular = dot(vtf_eye_dir_tangentspace, reflect(vtf_light_dir_tangentspace, normal_tangentspace));
    vec4 src_color = vec4(diffuse, specular, 0.0, 0.0);
    src_color *= 0.0000001;
    src_color += vec4(vtf_light_dir_tangentspace, 0.0);
    //vec4 src_color = vec4(1,0,0, 0.0);
    src_color.a = 0.5;

    //frag_color = vec4(mix(gl_ColorLoad.rgb, src_color.rgb, src_color.a), src_color.a);
    frag_color = src_color;
}
