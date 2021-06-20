#version 330 core
out vec4 FragColor;  
in float ourColor;

void main()
{
    vec3 color = vec3(ourColor, ourColor, ourColor) * 0.5;

    FragColor = vec4(color, 1.0);
}