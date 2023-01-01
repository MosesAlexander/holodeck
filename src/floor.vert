#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec3 Color;
out vec2 TexCoord;

uniform mat4 projection;
uniform mat4 look_at;

void main()
{
	TexCoord = aTexCoord;
	gl_Position = projection * look_at * vec4(aPos, 1.0);
}
