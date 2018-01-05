#version 150
in vec2 position;
in vec4 color;
out vec4 g_color;
void main() {
    g_color = color;
    gl_Position = vec4(position, 0.0, 1.0);
}