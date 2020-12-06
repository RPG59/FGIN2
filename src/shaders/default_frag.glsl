#version 330es

layout (location=0) in vec4 a_Pos;

void main() {
    gl_Position(a_Pos);
}
