uniform sampler2D t;

void main() {
    vec4 samp = texture(t, vec2(0.5, 0.5));
    gl_Position = samp;
}