#version 330 core

//in vec3 Color;
uniform vec4 ourColor

out vec4 outColor;

void main()
{
    outColor = vec4(ourColor, 1.0f);
}