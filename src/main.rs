extern crate glfw;

use glfw::{Action, Context, Key, WindowEvent, Window, Glfw};
use std::sync::mpsc::Receiver;
use std::ffi::{CString, CStr};
use glfw::ffi::GLFWwindow;

mod gl {
	    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

struct Application {
	vertex_shader_ids: Vec<gl::types::GLuint>,
}

impl Application {
	fn new() -> Application {
		Application {vertex_shader_ids: Vec::new()}
	}
}

extern fn framebuffer_size_callback(window: *mut GLFWwindow, width: i32, height: i32) {
	unsafe {
		gl::Viewport(0,0,width,height);
	}
}

fn init_glfw_window() -> (glfw::Glfw, Window, Receiver<(f64, WindowEvent)>) {
		let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

		glfw.window_hint(glfw::WindowHint::ContextVersion(3,3));
		glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
		let (mut window, events) = glfw.create_window(800, 600, "MyOpenGL", glfw::WindowMode::Windowed)
			.expect("Failed to create GLFW window.");

		window.set_key_polling(true);
		window.make_current();

		// the supplied function must be of the type:
		// `&fn(symbol: &'static str) -> *const std::os::raw::c_void`
		// `window` is a glfw::Window
		gl::load_with(|s| window.get_proc_address(s) as *const _);

		// loading a specific function pointer
		gl::Viewport::load_with(|s| window.get_proc_address(s) as *const _);

		unsafe {
			gl::Viewport(0,0,800,600);
			gl::ClearColor(0.2, 0.3, 0.3, 1.0);
		}

		unsafe {
			glfw::ffi::glfwSetFramebufferSizeCallback(window.window_ptr(), Some(framebuffer_size_callback));
		}

		(glfw, window, events)
}

fn render_loop(mut glfw: Glfw, mut window: Window, events: Receiver<(f64, WindowEvent)>) {
		while !window.should_close() {
			unsafe {
				gl::Clear(gl::COLOR_BUFFER_BIT);
			}
			for (_, event) in glfw::flush_messages(&events) {
				handle_window_event(&mut window, event);
			}
			window.swap_buffers();
			glfw.poll_events();
		}
}

fn shader_from_source(app: &mut Application, source: &CStr, kind: gl::types::GLenum) {
	app.vertex_shader_ids.push(unsafe{gl::CreateShader(kind)});

	unsafe {
		gl::ShaderSource(*app.vertex_shader_ids.last().unwrap(), 1, &source.as_ptr(), std::ptr::null());
		gl::CompileShader(*app.vertex_shader_ids.last().unwrap());
	}
}

fn main() {
		let app = Application::new();
	    let glfw;
		let window;
		let events;

		(glfw, window, events) = init_glfw_window();

		render_loop(glfw, window, events);


}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
	match event {
		glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
			window.set_should_close(true)
		}

		glfw::WindowEvent::Key(Key::Up, _, Action::Press, _ ) => {
			unsafe {
				gl::ClearColor(1.0,0.2,0.2,1.0);
			}
		}
		glfw::WindowEvent::Key(Key::Left, _, Action::Press, _ ) => {
			unsafe {
				gl::ClearColor(0.2,0.5,1.0,1.0);
			}
		}
		glfw::WindowEvent::Key(Key::Right, _, Action::Press, _ ) => {
			unsafe {
				gl::ClearColor(0.5,1.0,0.2,1.0);
			}
		}
		glfw::WindowEvent::Key(Key::Down, _, Action::Press, _ ) => {
			unsafe {
				gl::ClearColor(0.2,0.2,0.2,1.0);
			}
		}
		_ => {}
	}
}
