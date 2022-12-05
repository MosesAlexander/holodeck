extern crate glfw;

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
    pub fn render_loop(&mut self) {
            while !self.window.should_close() {
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                for (_, event) in glfw::flush_messages(&self.events) {
                    handle_window_event(&mut self.window, event);
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
		_ => {}
	}
}
