extern crate glfw;

use core::num;
use std::ffi::{CString, CStr};
use glfw::{Action, Context, Key, WindowEvent, Window, Glfw};
use std::sync::mpsc::Receiver;
use glfw::ffi::GLFWwindow;

mod gl {
        include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

extern fn framebuffer_size_callback(_window: *mut GLFWwindow, width: i32, height: i32) {
	unsafe {
		gl::Viewport(0,0,width,height);
	}
}


pub struct Application {
    pub vertex_shader_ids: Vec<gl::types::GLuint>,
    pub fragment_shader_ids: Vec<gl::types::GLuint>,
    program_ids: Vec<gl::types::GLuint>,
    vaos: Vec<gl::types::GLuint>,
    glfw: Glfw,
    window: Window,
    events: Receiver<(f64, WindowEvent)>,
}

pub const VERTEX_SHADER: gl::types::GLenum = gl::VERTEX_SHADER;
pub const FRAGMENT_SHADER: gl::types::GLenum = gl::FRAGMENT_SHADER;

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len+1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe {
        CString::from_vec_unchecked(buffer)
    }
}

impl Application {
    pub fn new() -> Application {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(3,3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        let (mut window, mut events) = glfw.create_window(800, 600, "MyOpenGL", glfw::WindowMode::Windowed)
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

        Application {vertex_shader_ids: Vec::new(),
                    fragment_shader_ids: Vec::new(),
                    program_ids: Vec::new(),
                    vaos: Vec::new(),
                    glfw: glfw,
                    window:window,
                    events: events}
    }

    // Compiles the given shader and stores its id in the App's vector
    // In the Error case of Result the first string is our error message, the second string is the OpenGL error message
    pub fn compile_shader_from_source(&mut self, source: &CStr, kind: gl::types::GLenum) -> Result<(), String> {
        let shader_ids: &mut Vec<gl::types::GLuint>;
        if kind == VERTEX_SHADER {
            shader_ids = &mut self.vertex_shader_ids;
        } else if kind == FRAGMENT_SHADER {
            shader_ids = &mut self.fragment_shader_ids;
        } else {
            return Err("Invalid shader type!".to_string());
        }

        shader_ids.push(
            unsafe {
                gl::CreateShader(kind)
            });

        let mut success: gl::types::GLint = 1;

        unsafe {
            gl::ShaderSource(*shader_ids.last().unwrap(),
                                1, 
                                &source.as_ptr(),
                                std::ptr::null());
            
            gl::CompileShader(*shader_ids.last().unwrap());
            
            gl::GetShaderiv(*shader_ids.last().unwrap(),
                                    gl::COMPILE_STATUS,
                                    &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(*shader_ids.last().unwrap(),
                                gl::INFO_LOG_LENGTH, &mut len);
            }
            let error = create_whitespace_cstring_with_len(len as usize);

            // Write shader log into error
            unsafe {
                gl::GetShaderInfoLog(*shader_ids.last().unwrap(), 
                                    len,
                                    std::ptr::null_mut(),
                                    error.as_ptr() as *mut gl::types::GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(())
    }

    pub fn create_and_link_program_vert_frag_shaders(&mut self, vert_shader_id: u32, frag_shader_id: u32) -> Result<(),String> {
        unsafe {
            let program_id: gl::types::GLuint = gl::CreateProgram();
            let mut len: gl::types::GLint = 0;

            gl::AttachShader(program_id, vert_shader_id);
            gl::AttachShader(program_id, frag_shader_id);
            gl::LinkProgram(program_id);


            let mut success: gl::types::GLint = 1;
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);

            if success == 0 {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);

                let error = create_whitespace_cstring_with_len(len as usize);

                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );

                return Err(error.to_string_lossy().into_owned());
            }

            gl::DetachShader(program_id, vert_shader_id);
            gl::DetachShader(program_id, frag_shader_id);

            self.program_ids.push(program_id);
        }

        Ok(())
    }

    pub fn use_program_at_index(&self, idx: usize) {
        unsafe {
            gl::UseProgram(self.program_ids[idx]);
        }
    }



    pub fn generate_buffers_triangle(&mut self, vertices: &Vec<f32>) {
        let mut vbo: gl::types::GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut vbo);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, //size of data in bytes
                vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0); //unbind buffer
        }

        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // specify data layout for attribute 0
            // this is layout (location = 0) in the vertex shader
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0, // index of the generic vertex attribute (layout (location = 0))
                3, // the number of components per generic 
                gl::FLOAT, // data type
                gl::FALSE, // normalized int-to-float conversion
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null() // offset of the first component
            );

            // unbind vbo and vao just for correctness, this is not really needed
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        self.vaos.push(vao);

    }

    pub fn generate_indexed_triangles(&mut self, vertices: &Vec<f32>,
                                   indexes_two_triangles: &Vec<u32>,
                                   vertices_third_triangle: &Vec<f32>,
                                   indexes_third_triangle: &Vec<u32>) {
        let mut vao: gl::types::GLuint = 0;
        let mut vbo: gl::types::GLuint = 0;
        let mut ebo: gl::types::GLuint = 0;
        let mut vao2: gl::types::GLuint = 0;
        let mut vbo2: gl::types::GLuint = 0;
        let mut ebo2: gl::types::GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );

            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indexes_two_triangles.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indexes_two_triangles.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            
            gl::VertexAttribPointer(0, 3, 
                gl::FLOAT,
                gl::FALSE,
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null()
            );
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(1, 3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid);
            gl::EnableVertexAttribArray(1);

            gl::GenVertexArrays(1, &mut vao2);
            gl::BindVertexArray(vao2);
            
            gl::GenBuffers(1, &mut vbo2);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo2);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices_third_triangle.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices_third_triangle.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );

            gl::GenBuffers(1, &mut ebo2);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo2);

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indexes_third_triangle.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indexes_third_triangle.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            
            gl::VertexAttribPointer(0, 3,
                gl::FLOAT, 
                gl::FALSE, 
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint, 
                std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(1, 3,
                    gl::FLOAT,
                    gl::FALSE,
                    (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
                    (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid);
            gl::EnableVertexAttribArray(1);
        }

        self.vaos.push(vao);
        self.vaos.push(vao2);
    }

    pub fn render_loop(&mut self) {
        /* 
            let mut uniColor: gl::types::GLint;
            unsafe {
                uniColor = gl::GetUniformLocation(self.program_ids[1], CString::new("triangleColor".to_string()).unwrap().as_ptr()); 
            } */

            //0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
            //-0.5, 0.0, 0.0, 1.0, 00.0, 0.0,
            //-0.25, 0.5, 0.0, 0.0, 0.0, 1.0,
            //0.25, 0.5, 0.0, 0.0, 1.0, 0.0,
            let mut color1: gl::types::GLint;
            let mut color2: gl::types::GLint;
            let mut color3: gl::types::GLint;
            let mut color4: gl::types::GLint;
            let mut color5: gl::types::GLint;
            let mut color1_green = 1.0;
            let mut color2_red = 1.0;
            let mut color3_blue = 1.0;
            let mut color4_green = 1.0;
            let mut common_gradient = 1.0;
            let mut gradient1 = 0.0;
            let mut gradient2 = 0.5;
            let mut gradient3 = 1.0;
            let mut sign1 = 1.0;
            let mut sign2 = 1.0;
            let mut sign3 = 1.0;

            unsafe {
                color1 = gl::GetUniformLocation(self.program_ids[0], CString::new("color1".to_string()).unwrap().as_ptr());
                color2 = gl::GetUniformLocation(self.program_ids[0], CString::new("color2".to_string()).unwrap().as_ptr());
                color3 = gl::GetUniformLocation(self.program_ids[0], CString::new("color3".to_string()).unwrap().as_ptr());
                color4 = gl::GetUniformLocation(self.program_ids[0], CString::new("color4".to_string()).unwrap().as_ptr());
                color5 = gl::GetUniformLocation(self.program_ids[0], CString::new("color5".to_string()).unwrap().as_ptr());
            }

            let mut num_attributes = 0;;
            unsafe {
                gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut num_attributes);
            }

            println!("Number of vertex attributes: {}", num_attributes);
            let mut component: f32 = 0.0;
            let factor = 0.01;
            let mut sign = 1.0;

            while !self.window.should_close() {
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                for (_, event) in glfw::flush_messages(&self.events) {
                    handle_window_event(&mut self.window, event);
                }

                unsafe {
                    /* 
                    gl::BindVertexArray(self.vaos[0]);
                    gl::DrawArrays(
                        gl::TRIANGLES,
                        0, // starting index in the enabled arrays
                        3 // number of indices to be rendered
                    );
                    */
                    self.use_program_at_index(0);
                    gl::BindVertexArray(self.vaos[0]);
                    // grbg
                    if gradient1 <= 0.0 || gradient1 >= 1.0 {
                        sign1*=-1.0;
                    }
                    if gradient2 <= 0.0 || gradient2 >= 1.0 {
                        sign2*=-1.0;
                    }
                    if gradient3 <= 0.0 || gradient3 >= 1.0 {
                        sign3*=-1.0;
                    }
                    //R: g3g2g1
                    //G: g1g3g2
                    //B: g1g2g3

                    if common_gradient <= 0.0 || common_gradient >= 1.0 {
                        sign*=-1.0;
                    }
                    gradient1 = gradient1 + (0.01*sign1);
                    gradient2 = gradient2 + (0.01*sign2);
                    gradient3 = gradient3 + (0.01*sign3);
                    common_gradient = common_gradient + (0.02*sign);
                    
                    gl::Uniform3f(color1, gradient1, gradient3, gradient2);
                    gl::Uniform3f(color2, gradient3, gradient2, gradient1);
                    gl::Uniform3f(color3, gradient1, gradient2, gradient3);
                    gl::Uniform3f(color4, gradient1, gradient3, gradient2);
                    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
                    gl::BindVertexArray(0);
                    self.use_program_at_index(1);
                    /* 
                    component+=factor*sign;
                    if component >= 1.0 || component <= 0.0 {
                        sign*=-1.0;
                    }
                    gl::Uniform3f(uniColor, component, component, component);
                    */

                    gl::BindVertexArray(self.vaos[1]);
                    gl::Uniform3f(color5, common_gradient, common_gradient, common_gradient);
                    gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());
                    gl::BindVertexArray(0);
                }

                self.window.swap_buffers();
                self.glfw.poll_events();
            }
    }

}

impl Drop for Application {
    fn drop(&mut self) {
        // Delete the shaders
        for shader_id in self.vertex_shader_ids.iter() {
            unsafe {
                gl::DeleteShader(*shader_id);
            }
        }

        for shader_id in self.fragment_shader_ids.iter() {
            unsafe {
                gl::DeleteShader(*shader_id);
            }
        }

        for program in self.program_ids.iter() {
            unsafe {
                gl::DeleteProgram(*program);
            }
        }

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
		glfw::WindowEvent::Key(Key::W, _, Action::Press, _ ) => {
			unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
			}
		}
		glfw::WindowEvent::Key(Key::F, _, Action::Press, _ ) => {
			unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL)
			}
		}
		_ => {}
	}
}
