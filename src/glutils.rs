
extern crate native;
extern crate glfw;
extern crate gl;

use std::ptr;
use std::str;
use std::io::File;
use std::io::BufferedReader;

use gl::types::*;

/*
 * 0 --- 3
 * |\    |
 * | \   |
 * |  \  |
 * |   \ |
 * |    \|
 * 1 --- 2
 */

pub static VERTEX_DATA: [GLfloat, ..16] = [
	-1.0,  1.0, 0.0, 1.0, //0
	-1.0, -1.0, 0.0, 0.0, //1
	 1.0,  1.0, 1.0, 1.0, //3
	 1.0, -1.0, 1.0, 0.0, //2
];

pub fn compile_shader(src: &str, ty: GLenum) -> GLuint {
	let shader = gl::CreateShader(ty);
	unsafe {
		src.with_c_str(|ptr| gl::ShaderSource(shader, 1, &ptr, ptr::null()));
		gl::CompileShader(shader);

		let mut status = gl::FALSE as GLint;
		gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

		if status != (gl::TRUE as GLint) {
			let mut len = 0;
			gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
			let mut buf = Vec::from_elem(len as uint - 1, 0u8);
			gl::GetShaderInfoLog(shader, len, ptr::mut_null(),
				buf.as_mut_ptr() as *mut GLchar);
			fail!("{}", str::from_utf8(buf.as_slice()).expect(
					"ShaderInfoLog not valid utf8"));
		}
	}
	shader
}

pub fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
	let program = gl::CreateProgram();
	gl::AttachShader(program, vs);
	gl::AttachShader(program, fs);
	gl::LinkProgram(program);
	unsafe {
		let mut status = gl::FALSE as GLint;
		gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

		if status != (gl::TRUE as GLint) {
			let mut len: GLint = 0;
			gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
			let mut buf = Vec::from_elem(len as uint - 1, 0u8);
			gl::GetProgramInfoLog(program, len, ptr::mut_null(),
				buf.as_mut_ptr() as *mut GLchar);
			fail!("{}", str::from_utf8(buf.as_slice()).expect(
					"ProgramInfoLog not valid utf8"));
		}
	}
	program
}


fn read_file(file_path: &Path, content: &mut String) -> () {
	let mut file = BufferedReader::new(File::open(file_path));
	for line_iter in file.lines() {
		let line: String = match line_iter {
			Ok(v) => v,
			Err(e) => fail!("Error: {}", e)
		};

		content.push_str(
			match str::from_utf8(line.as_bytes()) {
				Some(v) => v,
				None => ""
			}
		);
	}
}

pub fn load_program(vs: &str, fs: &str) -> GLuint {
	let vertex_path = Path::new(vs);
	let mut vertex_shader = box String::new();
	read_file(&vertex_path, vertex_shader);
	let fragment_path = Path::new(fs);
	let mut fragment_shader = box String::new();
	read_file(&fragment_path, fragment_shader);


	let vs = ::glutils::compile_shader(
	match str::from_utf8(vertex_shader.as_bytes()) {
		Some(v) => v,
		None => ""
	}, gl::VERTEX_SHADER);
	let fs = ::glutils::compile_shader(
		match str::from_utf8(fragment_shader.as_bytes()) {
			Some(v) => v,
			None => ""
		}, gl::FRAGMENT_SHADER);

    let program = ::glutils::link_program(vs, fs);
	gl::DeleteShader(fs);
	gl::DeleteShader(vs);

	return program;
}
