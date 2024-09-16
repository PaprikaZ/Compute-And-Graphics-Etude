#version 450

layout (location = 0) in vec3 inVertexPosition;
layout (location = 1) in vec3 inVertexNormal;
layout (location = 2) in vec3 inVertexColor;

layout (location = 0) out vec3 outColor;

layout (push_constant) uniform constants
{
  mat4 render_matrix;
} pushConstants;

void main() 
{	
	gl_Position = pushConstants.render_matrix * vec4(inVertexPosition, 1.0f);
	outColor = inVertexColor;
}
