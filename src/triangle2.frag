#version 330 core

in vec3 Color;
in vec2 TexCoord;

uniform sampler2D texture1;
uniform sampler2D texture2;
uniform float mixvalue = 0.2;

out vec4 outColor;

void main()
{
    outColor = mix(texture(texture1, TexCoord) * vec4(Color, 1.0), texture(texture2, TexCoord), mixvalue);
}