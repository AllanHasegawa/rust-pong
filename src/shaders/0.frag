#version 150
in vec2 position;
out vec4 out_color;

varying vec2 quad_coord;

void main() {
	if (gl_FragCoord.y > 300) {
		out_color = vec4(quad_coord.x, 0.0, 0.0, 1.0);
	} else {
		out_color = vec4(0.0, 0.0, 1.0, 1.0);
	}
}

