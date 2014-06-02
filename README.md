rust-pong
=========

A VERY simple Pong-like game made in Rust Language with OpenGL/GLFW.

# Dependencies

* Nightly rustc (0.11.0-pre-nightly (25951b2 2014-05-30 00:31:44 -0700))
* glfw-rs (https://github.com/bjz/glfw-rs)
* gl-rs (https://github.com/bjz/gl-rs)
* cgmath-rs (https://github.com/bjz/cgmath-rs)
* Makefile from https://github.com/bvssvni/rust-empty

# Playing

* `W,S` controls the player paddle?
* Make sure the directory `src/shaders` is inside the `bin` (like, `bin/shaders`, the code will try to open `shaders/0.vert` at runtime)
