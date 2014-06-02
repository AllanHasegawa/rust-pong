
use cgmath::point::*;

pub struct GameState {
	pub p1_pady: f32,
	pub p2_pady: f32,
	pub ball_center: Point2<f32>,
	pub ball_velocity: Point2<f32>,
	pub pad_moving_down: bool,
	pub pad_moving_up: bool,
	pub p1_score: i32,
	pub p2_score: i32,
	pub random_value: f32,
}

fn min(v1: f32, v2: f32) -> f32 {
	    if v1 < v2 { v1 } else { v2 }
}
fn max(v1: f32, v2: f32) -> f32 {
	    if v1 > v2 { v1 } else { v2 }
}

static BALL_RADIUS: f32 = 0.02;
static PAD_SIZE: f32 = 0.15;
static PAD_SPEED: f32 = 0.5;
static P1_X_MIN: f32 = 0.90;
static P1_X_MAX: f32 = 0.95;
static P2_X_MIN: f32 = 0.05;
static P2_X_MAX: f32 = 0.10;
static BALL_MAX_X_SPEED: f32 = 5.;

impl GameState {
	pub fn new() -> GameState {
		GameState {
			p1_pady: 0.5,
			p2_pady: 0.5,
			ball_center: Point2 {x: 0.5, y: 0.5 },
			ball_velocity: Point2{x: 0.3, y: 0.0},
			pad_moving_down: false,
			pad_moving_up: false,
			p1_score: 0,
			p2_score: 0,
			random_value: 0.0,
		}
	}

	pub fn update(&mut self, delta_time: f32) {
		if self.pad_moving_up {
			self.p1_pady += PAD_SPEED*delta_time;
		}
		if self.pad_moving_down {
			self.p1_pady -= PAD_SPEED*delta_time;
		}
		// [TODO] min/max?
		self.p1_pady = min(1., self.p1_pady);
		self.p1_pady = max(0., self.p1_pady);

		// AI level 2
		self.p2_pady += 0.3*delta_time;
		if self.p2_pady > 1. { self.p2_pady = 0.; }
		// AI level 9001
		self.p2_pady = self.ball_center.y;
		self.p2_pady = min(1., self.p2_pady);
		self.p2_pady = max(0., self.p2_pady);


		self.ball_center.x += self.ball_velocity.x*delta_time;
		self.ball_center.y += self.ball_velocity.y*delta_time;
		let bvx = self.ball_velocity.x;
		let bcx = self.ball_center.x;
		let bcy = self.ball_center.y;
		let p1y = self.p1_pady;
		let p2y = self.p2_pady;

		if bcy > 1.0-BALL_RADIUS {
			self.ball_velocity.y *= -1.;
		} else
		if bcy < BALL_RADIUS {
			self.ball_velocity.y *= -1.;
		}
		if bvx > 0. && bcx > P1_X_MIN-BALL_RADIUS
				&& bcy > p1y-BALL_RADIUS-PAD_SIZE
				&& bcy < p1y+BALL_RADIUS+PAD_SIZE {
			self.ball_velocity.x *= -1.05;
			self.ball_velocity.y = (bcy-p1y)*1.5;
		}
		if bvx < 0. && bcx < P2_X_MAX+BALL_RADIUS
				&& bcy > p2y-BALL_RADIUS-PAD_SIZE
				&& bcy < p2y+BALL_RADIUS+PAD_SIZE {
			self.ball_velocity.x *= -1.05;
			self.ball_velocity.y = (bcy-p2y)*1.5;
		}
		self.ball_velocity.x = min(BALL_MAX_X_SPEED, self.ball_velocity.x);

		if self.ball_center.x > 1.1 {
			self.p1_score += 1;
			self.reset();
		} else
		if self.ball_center.x < -0.1 {
			self.p2_score += 1;
			self.reset();
		}
	}

	pub fn reset(&mut self) {
		let n_gs = GameState::new();
		self.p1_pady = n_gs.p1_pady;
		self.p2_pady = n_gs.p2_pady;
		self.ball_center = n_gs.ball_center;
		self.ball_velocity = n_gs.ball_velocity;
	}
}

