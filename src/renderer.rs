
extern crate glfw;
extern crate gl;

use gl::types::*;
use std::mem;
use std::ptr;


pub struct RendererState {
	pub p1_pady_loc: GLint,
	pub p2_pady_loc: GLint,
	pub program: GLuint,
	pub vao: GLuint,
	pub vbo: GLuint,
}

impl RendererState {
	pub fn new() -> RendererState {
		let program = ::glutils::load_program("shaders/0.vert", "shaders/0.frag");

		let mut vao = 0;
		let mut vbo = 0;

		let mut p1_pady_loc: GLint;
		let mut p2_pady_loc: GLint;

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
		}

		RendererState {
			p1_pady_loc: p1_pady_loc,
			p2_pady_loc: p2_pady_loc,
			program: program,
			vao: vao,
			vbo: vbo,
		}
	}

	pub fn update(&self, gs: &::logic::GameState) {
		gl::Uniform1f(self.p1_pady_loc, gs.p1_pady);
		gl::Uniform1f(self.p2_pady_loc, gs.p2_pady);
	}

	pub fn destroy(&self) {
		gl::DeleteProgram(self.program);
		unsafe {
			gl::DeleteBuffers(1, &self.vbo);
			gl::DeleteVertexArrays(1, &self.vao);
		}
	}
}
