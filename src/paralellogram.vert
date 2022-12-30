#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

uniform vec3 color1;
uniform vec3 color2;
uniform vec3 color3;
uniform vec3 color4;

out vec3 Color;

void main()
{
	if (aPos.x == 0.0 && aPos.y == 0.0) {
		Color = color1;
	} else if (aPos.x == -0.5 && aPos.y == 0.0) {
		Color = color2;
	} else if (aPos.x == -0.25 && aPos.y ==  0.5) {
		Color = color3;
	} else if (aPos.x == 0.25 && aPos.y == 0.5) {
		Color = color4;
	}

	gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
