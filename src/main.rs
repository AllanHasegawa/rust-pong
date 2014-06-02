#![feature(globs)]

extern crate native;
extern crate glfw;
extern crate gl;
extern crate time;
extern crate cgmath;


use glfw::Context;
use gl::types::*;
use std::mem;
use std::ptr;

mod logic;
mod glutils;
mod renderer;

static WND_TITLE: &'static str = "rust-pong :: FPS: ";

#[start]
fn start(argc: int, argv: **u8) -> int {
	native::start(argc, argv, main)
}

fn main() {

	// Init Game State
	let mut game_state = ::logic::GameState::new();


	// Init Window/IO
	let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

	glfw.window_hint(glfw::ContextVersion(3, 2));
    glfw.window_hint(glfw::OpenglForwardCompat(true));
	
	let (window, events) = glfw.create_window(800, 600,
		WND_TITLE, glfw::Windowed).expect("Failed to create GLFW window.");

	window.set_key_polling(true);
	window.make_current();

	gl::load_with(|s| glfw.get_proc_address(s));


	// Init OpenGL
	let renderer_state = ::renderer::RendererState::new();


	// Start Main Loop
	let mut start_time = time::precise_time_ns();
	let ns_to_s: f32 = 1.0/1000000000.0;
	let mut frames = 0;
	let mut frames_interval: f32 = 0.;
	
	while !window.should_close() {
		// Main logic
		glfw.poll_events();
		for (_, event) in glfw::flush_messages(&events) {
			handle_window_event(&window, event);
		}
		let now_time =  time::precise_time_ns();
		let delta_time = ((now_time-start_time) as f32)*ns_to_s;
		start_time = now_time;
		game_state.update(delta_time);

		// Rendering
		renderer_state.update(&game_state);
		gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
		window.swap_buffers();

		frames += 1;
		frames_interval += delta_time;
		if frames_interval > 3. {
			let mut title = String::new();
			title.push_str(WND_TITLE.to_str().as_slice());
			title.push_str(
				((frames as f32)/frames_interval).to_str().as_slice());
			window.set_title(title.as_slice());
			frames_interval = 0.;
			frames = 0;
		}
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
