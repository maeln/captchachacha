#version 330

in vec4 oVertex;
in vec4 oNormal;
in vec4 fragPos;

out vec4 Color;

uniform float time;
uniform float color;

// fifth-order polynomial approximation of Turbo based on:
// https://observablehq.com/@mbostock/turbo
vec3 turbo(float x) {
    float r = 0.1357 + x * ( 4.5974 - x * ( 42.3277 - x * ( 130.5887 - x * ( 150.5666 - x * 58.1375 ))));
	float g = 0.0914 + x * ( 2.1856 + x * ( 4.8052 - x * ( 14.0195 - x * ( 4.2109 + x * 2.7747 ))));
	float b = 0.1067 + x * ( 12.5925 - x * ( 60.1097 - x * ( 109.0745 - x * ( 88.5066 - x * 26.8183 ))));
    return vec3(r,g,b);
}

void main() {
    vec3 lightPos = vec3(sin(time), cos(time), sin(time+3.1415));
    vec3 lightDir = normalize(lightPos - fragPos.xyz);
    
    float amb = 0.2;
    vec3 basecol = turbo(color);

    float diff = max(dot(oNormal, vec4(lightDir, 1.0)), 0.0);
    vec3 diffuse = diff * vec3(1.0);
    
    vec3 res = (amb + diffuse) * basecol;
    Color = vec4(res, 1.0);
}
