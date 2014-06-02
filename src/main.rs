#![feature(globs)]

extern crate native;
extern crate glfw;
extern crate gl;
extern crate time;

use glfw::Context;
use gl::types::*;
use std::mem;
use std::ptr;

mod glutils;
mod logic;

#[start]
fn start(argc: int, argv: **u8) -> int {
	native::start(argc, argv, main)
}

fn main() {

	let mut game_state = ::logic::GameState::new();

	let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

	glfw.window_hint(glfw::ContextVersion(3, 2));
    glfw.window_hint(glfw::OpenglForwardCompat(true));
	
	let (window, events) = glfw.create_window(800, 600,
		"Title :)", glfw::Windowed).expect("Failed to create GLFW window.");

	window.set_key_polling(true);
	window.make_current();

	gl::load_with(|s| glfw.get_proc_address(s));

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
		gl::Uniform1f(p1_pady_loc, game_state.p1_pady);
		p2_pady_loc = "p2_pady".with_c_str(
				|ptr| gl::GetUniformLocation(program, ptr));
		gl::Uniform1f(p2_pady_loc, game_state.p2_pady);
	}

	let mut start_time = time::precise_time_ns();
	let ns_to_s: f32 = 1.0/1000000000.0;
	let mut frames = 0;
	let mut frames_interval: f32 = 0.;
	
	while !window.should_close() {
		glfw.poll_events();
		for (_, event) in glfw::flush_messages(&events) {
			handle_window_event(&window, event);
		}
		gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);

		// Swap buffers
		window.swap_buffers();

		let now_time =  time::precise_time_ns();
		let delta_time = ((now_time-start_time) as f32)*ns_to_s;
		start_time = now_time;
		game_state.update(delta_time);
		gl::Uniform1f(p1_pady_loc, game_state.p1_pady);
		gl::Uniform1f(p2_pady_loc, game_state.p2_pady);

		frames += 1;
		frames_interval += delta_time;
		if frames_interval > 3. {
			let mut title = String::new();
			title.push_str("rust-pong :: FPS: ".to_str().as_slice());
			title.push_str(
				((frames as f32)/frames_interval).to_str().as_slice());
			window.set_title(title.as_slice());
			frames_interval = 0.;
			frames = 0;
		}
	}

	gl::DeleteProgram(program);
	unsafe {
		gl::DeleteBuffers(1, &vbo);
		gl::DeleteVertexArrays(1, &vao);
	}
}

fn handle_window_event(window: &glfw::Window, event: glfw::WindowEvent) {
	match event {
		glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
			window.set_should_close(true);
		}
		_ => {}
	}
}
