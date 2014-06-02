#version 150

#define BALL_RADIUS 0.02
#define P1_X_MIN 0.90
#define P1_X_MAX 0.95
#define P2_X_MIN 0.05
#define P2_X_MAX 0.10
#define PAD_SIZE 0.15

in vec2 position;
out vec4 out_color;

varying vec2 quad_coord;

uniform float p1_pady;
uniform float p2_pady;
uniform vec2 ball_center;

void main() {
	vec3 color = vec3(0,0,0);

	vec2 ball_zero = ball_center - quad_coord;
	if (BALL_RADIUS*BALL_RADIUS >  dot(ball_zero, ball_zero)) {
		color += vec3(quad_coord.x/2.,quad_coord.x,quad_coord.x*2);
	} else
	if (quad_coord.x > P1_X_MIN && quad_coord.x < P1_X_MAX
			&& quad_coord.y > p1_pady-PAD_SIZE
			&& quad_coord.y < p1_pady+PAD_SIZE) {
		color += vec3(0.5,0,0);
	} else
	if (quad_coord.x > P2_X_MIN && quad_coord.x < P2_X_MAX
			&& quad_coord.y > p2_pady-PAD_SIZE
			&& quad_coord.y < p2_pady+PAD_SIZE) {
		color += vec3(0.5,0,0);
	}

	out_color = vec4(color, 1.0);
}

