#![feature(globs)]

extern crate native;
extern crate glfw;
extern crate gl;

use glfw::Context;
use gl::types::*;
use std::mem;
use std::ptr;

mod glutils;

#[start]
fn start(argc: int, argv: **u8) -> int {
	native::start(argc, argv, main)
}

fn main() {

	println!("Hello World!");

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

		let quad_coord_attr = 
			"quad_coord_in".with_c_str(
				|ptr| gl::GetAttribLocation(program, ptr));
		gl::EnableVertexAttribArray(quad_coord_attr as GLuint);
		gl::VertexAttribPointer(quad_coord_attr as GLuint, 2, gl::FLOAT,
			gl::FALSE as GLboolean,
			4*4, ptr::null().offset(2*4)); // [TODO] sizeof(GLfloat) ?
	}
	
	while !window.should_close() {
		glfw.poll_events();
		for (_, event) in glfw::flush_messages(&events) {
			handle_window_event(&window, event);
		}
		gl::ClearColor(0.3, 0.3, 0.3, 1.0);
		gl::Clear(gl::COLOR_BUFFER_BIT);

		//gl::DrawArrays(gl::TRIANGLES, 0, 3);
		gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);

		// Swap buffers
		window.swap_buffers();
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
