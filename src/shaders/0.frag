#version 150
in vec2 position;
out vec4 out_color;

varying vec2 quad_coord;

uniform float p1_pady;
uniform float p2_pady;

void main() {
	vec3 color = vec3(0,0,0);

	if (quad_coord.x > 0.90 && quad_coord.x < 0.95
			&& quad_coord.y > p1_pady-0.15 && quad_coord.y < p1_pady+0.15) {
		color += vec3(0.5,0,0);
	} else
	if (quad_coord.x > 0.05 && quad_coord.x < 0.10
			&& quad_coord.y > p2_pady-0.15 && quad_coord.y < p2_pady+0.15) {
		color += vec3(0.5,0,0);
	}

	out_color = vec4(color, 1.0);
}

