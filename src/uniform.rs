use crate::gl;
use std::ffi::{CString, c_int, c_void};

// Every uniform is associated with a program
pub struct UniformDescriptor {
    uniform_id: gl::types::GLint,
}

impl UniformDescriptor {
    pub fn new(program_id: gl::types::GLuint, uniform_name: &str) -> UniformDescriptor {
        let mut uniform_id = 0;
        
        unsafe {
            uniform_id = gl::GetUniformLocation(program_id, CString::new(uniform_name.to_string()).unwrap().as_ptr());
        }

        UniformDescriptor { uniform_id: uniform_id }
    }
}