
extern crate native;
extern crate glfw;

use glfw::Context;

fn main() {

	println!("Hello World!");

	let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
	
	let (window, events) = glfw.create_window(300, 300,
		"Title :)", glfw::Windowed).expect("Failed to create GLFW window.");

	window.set_key_polling(true);
	window.make_current();
	
	while !window.should_close() {
		glfw.poll_events();
		for (_, event) in glfw::flush_messages(&events) {
			handle_window_event(&window, event);
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
