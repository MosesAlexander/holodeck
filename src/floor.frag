#version 330 core

in vec2 TexCoord;

uniform sampler2D texture1;

out vec4 outColor;

void main()
{
    outColor = texture(texture1, TexCoord);
}