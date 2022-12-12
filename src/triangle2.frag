#version 330 core

in vec3 Color;
in vec2 TexCoord;

uniform sampler2D outTexture;

out vec4 outColor;

void main()
{
    outColor = texture(outTexture, TexCoord);
}