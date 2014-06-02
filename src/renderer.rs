
extern crate glfw;
extern crate gl;
extern crate rand;

use rand::Rng;

use gl::types::*;
use std::mem;
use std::ptr;

pub struct RendererState {
	pub p1_pady_loc: GLint,
	pub p2_pady_loc: GLint,
	pub ball_center_loc: GLint,
	pub sepia_value_loc: GLint,
	pub noise_value_loc: GLint,
	pub scratch_value_loc: GLint,
	pub inner_vignetting_loc: GLint,
	pub outer_vignetting_loc: GLint,
	pub random_value_loc: GLint,
	pub time_lapse_loc: GLint,
	pub program: GLuint,
	pub vao: GLuint,
	pub vbo: GLuint,
}

impl RendererState {
	pub fn new() -> RendererState {
		let program = ::glutils::load_program("shaders/0.vert", "shaders/1.frag");

		let mut vao = 0;
		let mut vbo = 0;

		let mut p1_pady_loc: GLint;
		let mut p2_pady_loc: GLint;
		let mut ball_center_loc: GLint;
		let mut sepia_value_loc: GLint;
		let mut noise_value_loc: GLint;
		let mut scratch_value_loc: GLint;
		let mut inner_vignetting_loc: GLint;
		let mut outer_vignetting_loc: GLint;
		let mut random_value_loc: GLint;
		let mut time_lapse_loc: GLint;

		unsafe {
			gl::GenVertexArrays(1, &mut vao);
			gl::BindVertexArray(vao);

			gl::GenBuffers(1, &mut vbo);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::BufferData(gl::ARRAY_BUFFER,
			(::glutils::VERTEX_DATA.len()
			 * mem::size_of::<GLfloat>()) as GLsizeiptr,
			 mem::transmute(&::glutils::VERTEX_DATA[0]),
			 gl::STATIC_DRAW);

			gl::UseProgram(program);
			"out_color".with_c_str(
				|ptr| gl::BindFragDataLocation(program, 0, ptr));


			let pos_attr = 
				"position".with_c_str(|ptr| gl::GetAttribLocation(program, ptr));
			gl::EnableVertexAttribArray(pos_attr as GLuint);
			gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT,
			gl::FALSE as GLboolean,
			4*4, ptr::null()); // [TODO] sizeof(GLfloat) ?

			let quad_coord_loc = 
				"quad_coord_in".with_c_str(
					|ptr| gl::GetAttribLocation(program, ptr));
			gl::EnableVertexAttribArray(quad_coord_loc as GLuint);
			gl::VertexAttribPointer(quad_coord_loc as GLuint, 2, gl::FLOAT,
			gl::FALSE as GLboolean,
			4*4, ptr::null().offset(2*4)); // [TODO] sizeof(GLfloat) ?

			p1_pady_loc = "p1_pady".with_c_str(
				|ptr| gl::GetUniformLocation(program, ptr));
			gl::Uniform1f(p1_pady_loc, 0.0);
			p2_pady_loc = "p2_pady".with_c_str(
				|ptr| gl::GetUniformLocation(program, ptr));
			gl::Uniform1f(p2_pady_loc, 0.0);
			ball_center_loc = "ball_center".with_c_str(
				|ptr| gl::GetUniformLocation(program, ptr));
			gl::Uniform2f(ball_center_loc, 0.0, 0.0);
			sepia_value_loc = "SepiaValue".with_c_str(
				|ptr| gl::GetUniformLocation(program, ptr));
			noise_value_loc = "NoiseValue".with_c_str(
				|ptr| gl::GetUniformLocation(program, ptr));
			scratch_value_loc = "ScratchValue".with_c_str(
				|ptr| gl::GetUniformLocation(program, ptr));
			inner_vignetting_loc = "InnerVignetting".with_c_str(
				|ptr| gl::GetUniformLocation(program, ptr));
			outer_vignetting_loc = "OuterVignetting".with_c_str(
				|ptr| gl::GetUniformLocation(program, ptr));
			random_value_loc = "RandomValue".with_c_str(
				|ptr| gl::GetUniformLocation(program, ptr));
			time_lapse_loc = "TimeLapse".with_c_str(
				|ptr| gl::GetUniformLocation(program, ptr));
		}

		RendererState {
			p1_pady_loc: p1_pady_loc,
			p2_pady_loc: p2_pady_loc,
			ball_center_loc: ball_center_loc,
			sepia_value_loc: sepia_value_loc,
			noise_value_loc: noise_value_loc,
			scratch_value_loc: scratch_value_loc,
			inner_vignetting_loc: inner_vignetting_loc,
			outer_vignetting_loc: outer_vignetting_loc,
			random_value_loc: random_value_loc,
			time_lapse_loc: time_lapse_loc,
			program: program,
			vao: vao,
			vbo: vbo,
		}
	}

	pub fn update(&self, delta_time: f32, gs: &::logic::GameState) {
		gl::Uniform1f(self.p1_pady_loc, gs.p1_pady);
		gl::Uniform1f(self.p2_pady_loc, gs.p2_pady);
		gl::Uniform2f(self.ball_center_loc,
			gs.ball_center.x, gs.ball_center.y);
		gl::Uniform1f(self.sepia_value_loc, 1.0);
		gl::Uniform1f(self.noise_value_loc, 0.2);
		gl::Uniform1f(self.scratch_value_loc, 1.0);
		gl::Uniform1f(self.inner_vignetting_loc, 0.5);
		gl::Uniform1f(self.outer_vignetting_loc, 1.0);
		gl::Uniform1f(self.random_value_loc, gs.random_value);
		gl::Uniform1f(self.time_lapse_loc, delta_time*1000000.0);
	}
}

impl Drop for RendererState {
	fn drop(&mut self) {
		gl::DeleteProgram(self.program);
		unsafe {
			gl::DeleteBuffers(1, &self.vbo);
			gl::DeleteVertexArrays(1, &self.vao);
		}
	}
}
