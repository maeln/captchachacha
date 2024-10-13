#version 330

layout (location = 0) in vec3 Vertex;
layout (location = 1) in vec3 Normal;

out vec4 oVertex;
out vec4 oNormal;
out vec4 fragPos;

uniform float time;
uniform mat4 model;
uniform mat4 view;
uniform mat4 perspective;
uniform mat4 normalMat;

void main() {
	vec4 wv = perspective * view * model * vec4(Vertex, 1.0);
	// vec4 wn = normalMat * vec4(Normal, 1.0);
	vec4 wn = vec4(Normal, 1.0);
	fragPos = vec4(model * vec4(Vertex, 1.0));
	oVertex = wv;
	oNormal = wn;
	gl_Position = wv;
}
