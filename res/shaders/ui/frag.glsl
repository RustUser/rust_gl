#version 330 core

struct FillStrategy {
    int fillMethod;
    bool direction;
    float ratio;
};

uniform vec2 bottomLeft;
uniform vec2 size;
uniform bool enableCorner;
uniform float cornerRadius;

uniform bool texture0;
uniform sampler2D ourTexture;

uniform vec4 color;
uniform FillStrategy fillStrategy;

in vec2 currentPos;
in vec2 uv;

out vec4 FragColor;

bool perform_strategy() {
    if (fillStrategy.fillMethod == 0) {
        return false;
    } else if (fillStrategy.fillMethod == 1) {
        //Horizontal fill
        if (fillStrategy.direction) {
            if (currentPos.x <= fillStrategy.ratio)
            return true;
        } else {
            if (1 - currentPos.x <= fillStrategy.ratio)
            return true;
        }
    } else if (fillStrategy.fillMethod == 2) {
        if (fillStrategy.direction) {
            if (currentPos.y <= fillStrategy.ratio)
            return true;
        } else {
            if (1-currentPos.y <= fillStrategy.ratio)
            return true;
        }
    }

    return false;
}

void main()
{
    if (perform_strategy())
    discard;
    if (texture0) {
        FragColor = texture(ourTexture, uv) * color;
    } else
        FragColor = color;
}