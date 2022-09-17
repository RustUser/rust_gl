#version 330 core

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

out vec2 textureCoords;

layout(location = 1) in vec3 Position;
layout(location = 2) in vec2 Uv;

void main() {
    gl_Position = projection * view * model * vec4(Position, 1.0);
    textureCoords = Uv;
}