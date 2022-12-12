#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 2) in vec2 aTexCoord;
//in vec3 color;
uniform vec3 color1;
uniform vec3 color2;
uniform vec3 color3;
uniform vec3 color4;
uniform vec3 color5;
uniform vec3 position_offset;

out vec3 Color;
out vec2 TexCoord;

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
	} else {
		Color = vec3(aPos.x + position_offset.x, aPos.y+position_offset.y, aPos.z);
		TexCoord = aTexCoord;
	}

	//Color = color;
	gl_Position = vec4(aPos.x + position_offset.x, aPos.y+position_offset.y, aPos.z, 1.0);
}
