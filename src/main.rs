mod application;

use application::Application;
use std::ffi::{CString,CStr};


fn main() {
	let mut app = Application::new();

	match app.compile_shader_from_source(
		&CString::new(include_str!("/home/amo/work/rust-opengl/src/triangle.vert")).unwrap(),
			application::VERTEX_SHADER) {
				Ok(()) => {}
				Err(e) => {
					println!("ERROR: {}, exiting program", e);
					std::process::exit(1);
				}

			}
			
	match app.compile_shader_from_source(
			&CString::new(include_str!("/home/amo/work/rust-opengl/src/triangle.frag")).unwrap(),
				application::FRAGMENT_SHADER) {
			Ok(()) => {

			},
			Err(e) => {
				println!("ERROR: {}, exiting program", e);
				std::process::exit(1);
			}
	}

	match app.create_and_link_program_vert_frag_shaders(
				app.vertex_shader_ids[0], 
				app.fragment_shader_ids[0]) {
					Ok(()) => {

					},
					Err(e) => {
						println!("ERROR: {}, exiting program", e);
						std::process::exit(1);
					}
				}
	app.use_program_at_index(0);

	app.render_loop();
}

