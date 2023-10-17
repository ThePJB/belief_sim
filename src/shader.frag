#version 330 core
in vec4 colour;
in vec4 gl_FragCoord;

out vec4 frag_colour;

#define PI 3.1415926535897932384626433832795
#define ROOT2INV 0.70710678118654752440084436210484903928

uniform float time;


void main() {
    frag_colour = colour;
}