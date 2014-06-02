#![feature(globs)]

extern crate native;
extern crate glfw;
extern crate gl;
extern crate time;
extern crate cgmath;
extern crate rand;

use rand::Rng;

use glfw::Context;
use gl::types::*;
use std::mem;
use std::ptr;

mod logic;
mod glutils;
mod renderer;

static WND_TITLE: &'static str = "rust-pong";

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
	
	let mut rng = rand::isaac::IsaacRng::new_unseeded();
	while !window.should_close() {
		// Main logic
		glfw.poll_events();
		for (_, event) in glfw::flush_messages(&events) {
			handle_window_event(&window, event, &mut game_state);
		}
		let now_time =  time::precise_time_ns();
		let delta_time = ((now_time-start_time) as f32)*ns_to_s;
		start_time = now_time;
		game_state.update(delta_time);
		game_state.random_value = rng.gen_range(0.0, 1.0) as f32;
		println!("{}", game_state.random_value);

		// Rendering
		renderer_state.update(delta_time, &game_state);
		gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
		window.swap_buffers();

		frames += 1;
		frames_interval += delta_time;
		if frames_interval > 3. {
			let mut title = String::new();
			title.push_str(WND_TITLE.to_str().as_slice());
			title.push_str(" :: Score ");
			title.push_str(game_state.p1_score.to_str().as_slice());
			title.push_str("x");
			title.push_str(game_state.p2_score.to_str().as_slice());
			title.push_str(" :: FPS: ");
			title.push_str(
				((frames as f32)/frames_interval).to_str().as_slice());
			window.set_title(title.as_slice());
			frames_interval = 0.;
			frames = 0;
		}
	}
}

fn handle_window_event(window: &glfw::Window,
		event: glfw::WindowEvent, gs: &mut ::logic::GameState) {
	match event {
		glfw::KeyEvent(glfw::KeyW, _, glfw::Press, _) => {
			gs.pad_moving_up = true;
		}
		glfw::KeyEvent(glfw::KeyW, _, glfw::Release, _) => {
			gs.pad_moving_up = false;
		}
		glfw::KeyEvent(glfw::KeyS, _, glfw::Press, _) => {
			gs.pad_moving_down = true;
		}
		glfw::KeyEvent(glfw::KeyS, _, glfw::Release, _) => {
			gs.pad_moving_down = false;
		}
		glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
			window.set_should_close(true);
		}
		_ => {}
	}
}
