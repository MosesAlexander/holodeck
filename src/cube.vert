#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec3 Color;
out vec2 TexCoord;

uniform mat4 rotate_about_x;
uniform mat4 rotate_about_y;
uniform mat4 rotate_about_z;
uniform mat4 translate;
uniform mat4 projection;
uniform mat4 look_at;

void main()
{
	TexCoord = aTexCoord;
	gl_Position = projection * look_at * translate * rotate_about_x * rotate_about_y * rotate_about_z * vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
