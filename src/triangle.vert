#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 2) in vec2 aTexCoord;
//in vec3 color;
uniform vec3 color1;
uniform vec3 color2;
uniform vec3 color3;
uniform vec3 color4;

out vec3 Color;
out vec2 TexCoord;

uniform mat4 rotate_about_x;
uniform mat4 rotate_about_y;
uniform mat4 rotate_about_z;
uniform mat4 translate;
uniform mat4 projection;

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
		Color = vec3(aPos.x + translate[3][0], aPos.y + translate[3][1], aPos.z + translate[3][2]);
		TexCoord = aTexCoord;
		gl_Position = projection * translate * rotate_about_x * rotate_about_y * rotate_about_z * vec4(aPos.x, aPos.y, aPos.z, 1.0);
		return;
	}

	//Color = color;
	gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
