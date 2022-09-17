#version 330 core

uniform bool texture0;
uniform bool texture1;

uniform sampler2D sample0;
uniform sampler2D sample1;

in vec2 uv;

out vec4 Color;
uniform vec4 color;

void main()
{
    if (texture1) {
        //Color = vec4(0.0f, 0.5f, 1.0f, 1.0f) * color;
        //Color = texture2D(sample0, uv);
        //Color = vec4(uv, 0.0, 1.0);
    } else {
        Color = vec4(1.0f, 0.5f, 0.0f, 1.0f) * color;
    }
    Color = vec4(0.0, 0.0, 0.0, 1.0);
}