mod application;

use application::{Application, Shader, Program, FRAGMENT_SHADER, VERTEX_SHADER};
use std::ffi::{CString,CStr};


fn main() {
	let mut app = Application::new();


	let mut vert_shader = Shader::new(CString::new(include_str!("triangle.vert")).unwrap(),VERTEX_SHADER);
	let mut frag_shader1 = Shader::new(CString::new(include_str!("triangle3.frag")).unwrap(),FRAGMENT_SHADER);
	let mut frag_shader2 = Shader::new(CString::new(include_str!("triangle3.frag")).unwrap(),FRAGMENT_SHADER);

	match vert_shader.compile() {
		Ok(()) => {}
		Err(e) => {
			println!("ERROR: {}, exiting program", e);
			std::process::exit(1);
		}
	}

	match frag_shader1.compile() {
		Ok(()) => {}
		Err(e) => {
			println!("ERROR: {}, exiting program", e);
			std::process::exit(1);
		}
	}

	match frag_shader2.compile() {
		Ok(()) => {}
		Err(e) => {
			println!("ERROR: {}, exiting program", e);
			std::process::exit(1);
		}
	}

	let mut program1 = Program::new();
	let mut program2 = Program::new();

	program1.add_shader(&vert_shader);
	program1.add_shader(&frag_shader2);
	match program1.link_shaders() {
		Ok(()) => {},
		Err(e) => {
			println!("ERROR: {}, exiting program", e);
			std::process::exit(1);
		}
	}

	program2.add_shader(&vert_shader);
	program2.add_shader(&frag_shader2);
	match program2.link_shaders() {
		Ok(()) => {},
		Err(e) => {
			println!("ERROR: {}, exiting program", e);
			std::process::exit(1);
		}
	}

	app.add_program(&program1);
	app.add_program(&program2);
	//app.use_program_at_index(0);

	/* 
	let vertices: Vec<f32> = vec![
		-0.5, -0.5, 0.0,
		0.5, -0.5, 0.0,
		0.0, 0.5, 0.0,
	]; 

	app.generate_buffers_triangle(&vertices); */

	let vertices_indexed_two_triangles: Vec<f32> = vec![
		0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
		-0.5, 0.0, 0.0, 1.0, 00.0, 0.0,
		-0.25, 0.5, 0.0, 0.0, 0.0, 1.0,
		0.25, 0.5, 0.0, 0.0, 1.0, 0.0,
		0.5, 0.0, 0.0, 0.0, 0.0, 1.0,
	];

	let vertices_third_triangle: Vec<f32> = vec! [
		-0.15, 0.5, 0.0, 0.8, 0.8, 0.8,
		0.0, 0.75, 0.0, 0.3, 0.3, 0.3,
		0.15, 0.5, 0.0, 0.1, 0.1, 0.1,
	];


	let indices_two_triangles: Vec<u32> = vec! [
		0, 1, 2,
		2, 0, 3,
	];

	let indices_third_triangle: Vec<u32> = vec! [
		0, 1, 2,
	];

	app.generate_indexed_triangles(&vertices_indexed_two_triangles,
				&indices_two_triangles,
				&vertices_third_triangle,
				&indices_third_triangle);

	app.render_loop();
}

