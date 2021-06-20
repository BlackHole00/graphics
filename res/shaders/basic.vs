#version 330 core
layout (location = 0) in vec3 aPos;   // the position variable has attribute position 0
layout (location = 1) in vec2 aUV; // the color variable has attribute position 1
  
out vec2 ourUV; // output a color to the fragment shader

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

void main()
{
    gl_Position =  proj * view * model * vec4(aPos, 1.0);
    ourUV = aUV; // set ourColor to the input color we got from the vertex data
}