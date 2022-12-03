extern crate glfw;

use glfw::{Action, Context, Key};

mod gl {
	    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

fn main() {
	    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

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

		//window.

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
