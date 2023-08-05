in vec3 vtf_normal;
in vec2 vtf_uv;

uniform sampler2D tex;

layout(location = 0) out vec4 frag_color;

void main() {
    vec4 src_color = texture(tex, vtf_uv);
    src_color.a = 0.5;

    frag_color = vec4(mix(gl_ColorLoad.rgb, src_color.rgb, src_color.a), src_color.a);
}
