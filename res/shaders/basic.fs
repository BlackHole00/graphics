#version 330 core
out vec4 FragColor;  
in vec2 ourUV;

uniform sampler2D texture1;
uniform sampler2D texture2;
uniform float mixValue;

void main()
{
    FragColor = mix(texture(texture1, ourUV), texture(texture2, ourUV), mixValue);
}