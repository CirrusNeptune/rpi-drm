layout(location = 0) in vec4 in_position;







#include "test_triangle2_inc.glsl"
//out vec3 vtf_xyz;

void main() {
    gl_Position = in_position;
    vtf_xyz = in_position.xyz;
}
