layout(location = 0) out vec4 frag_color;

uniform sampler2D tex1;
uniform sampler2D tex2;
uniform sampler2D t;

void main() {
    frag_color = texture(tex1, vec2(0.5, 0.5)) + gl_FragCoord;
}
