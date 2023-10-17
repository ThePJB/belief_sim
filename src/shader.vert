#version 330 core
layout (location = 0) in vec3 in_pos;
layout (location = 1) in vec4 in_colour;

uniform mat4 projection;

// const mat4 projection = mat4(
//     1, 0, 0, 0,
//     0, -1, 0, 0,
//     0, 0, 1, 0,
//     0, 0, 0, 1
// );

out vec4 colour;
out vec2 uv;

void main() {
    colour = in_colour;
    gl_Position = projection * vec4(in_pos, 1.0);
}