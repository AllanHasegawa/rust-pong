
#version 150
in vec2 position;
out vec2 pos;

attribute vec2 quad_coord_in;
varying vec2 quad_coord;

void main() {
   quad_coord = quad_coord_in;
   gl_Position = vec4(position, 0.0, 1.0);
}
