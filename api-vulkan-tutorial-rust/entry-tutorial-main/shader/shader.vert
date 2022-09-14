#version 450

layout(location = 0) in vec2 inputPosition;
layout(location = 1) in vec3 inputColor;

layout(location = 0) out vec3 fragmentColor;


void main() {
    gl_Position = vec4(inputPosition, 0.0, 1.0);
    fragmentColor = inputColor;
}