#version 450

layout(binding = 0) uniform TransformD3ModelViewProjection {
    mat4 model;
    mat4 view;
    mat4 projection;
} main3DTransform;

layout(location = 0) in vec3 inputPosition;
layout(location = 1) in vec3 inputColor;
layout(location = 2) in vec2 inputTextureCoordinate;

layout(location = 0) out vec3 outputFragmentColor;
layout(location = 1) out vec2 outputFragmentTextureCoordinate;


void main() {
    gl_Position =
        main3DTransform.projection * main3DTransform.view * main3DTransform.model *
        vec4(inputPosition, 1.0);
    outputFragmentColor = inputColor;
    outputFragmentTextureCoordinate = inputTextureCoordinate;
}