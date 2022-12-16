use crate::gl;
use std::ffi::{CString};

// Every uniform is associated with a program
pub struct UniformDescriptor {
    uniform_shader_handle: gl::types::GLint,
}

pub struct Uniform3FParam(pub f32,pub f32,pub f32);

pub struct Uniform1IParam(pub i32);

pub struct Uniform1FParam(pub f32);

pub enum UniformPackedParam {
    Uniform3F(Uniform3FParam),
    Uniform1I(Uniform1IParam),
    Uniform1F(Uniform1FParam),
}

impl UniformDescriptor {
    pub fn new(program_id: gl::types::GLuint, uniform_name: &str) -> UniformDescriptor {
        let mut uniform_shader_handle = 0;
        
        unsafe {
            uniform_shader_handle = gl::GetUniformLocation(program_id, CString::new(uniform_name.to_string()).unwrap().as_ptr());
        }

        UniformDescriptor { uniform_shader_handle: uniform_shader_handle }
    }

    pub fn update(&mut self, packedParam: UniformPackedParam) {
        match packedParam {
            UniformPackedParam::Uniform1F(param) => {
                unsafe {
                    gl::Uniform1f(self.uniform_shader_handle, param.0);
                }
            },
            UniformPackedParam::Uniform3F(param) => {
                unsafe {
                    gl::Uniform3f(self.uniform_shader_handle, param.0, param.1, param.2);
                }
                
            },
            UniformPackedParam::Uniform1I(param) => {
                unsafe {
                    gl::Uniform1i(self.uniform_shader_handle, param.0);
                }
            }
        }
    }
}