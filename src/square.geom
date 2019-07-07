#version 150

layout(points) in;
layout(triangle_strip, max_vertices = 4) out;

in vec3 g_color[1];
out vec3 f_color;

uniform vec2 dims;
void main() {
    vec2 top_left = gl_in[0].gl_Position.xy;
    vec2 top_right = vec2(top_left.x + dims.x, top_left.y);
    vec2 bottom_left = vec2(top_left.x, top_left.y + dims.y);
    vec2 bottom_right = top_right + dims;

    f_color = g_color[0];

    gl_Position = vec4(top_left, 0.0, 1.0);
    EmitVertex();
    gl_Position = vec4(top_right, 0.0, 1.0);
    EmitVertex();
    gl_Position = vec4(bottom_left, 0.0, 1.0);
    EmitVertex();
    gl_Position = vec4(bottom_right, 0.0, 1.0);
    EmitVertex();
}