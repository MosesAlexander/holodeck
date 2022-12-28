use crate::gl;
use crate::program::create_whitespace_cstring_with_len;
use std::{ffi::CString, fs::read_to_string};

pub struct Shader {
    pub id: gl::types::GLuint,
    pub source: CString,
}

impl Shader {
    pub fn new(source: &str, kind: gl::types::GLenum) -> Shader {
        let source_string = CString::new(read_to_string(source).unwrap()).unwrap();
        let mut shader_id = 0;
        unsafe {
            shader_id = gl::CreateShader(kind);
        }

        Shader {
            id: shader_id,
            source: source_string,
        }
    }

    pub fn compile(&mut self) -> Result<(), String> {
        let mut success: gl::types::GLint = 1;

        unsafe {
            gl::ShaderSource(self.id, 1, &self.source.as_ptr(), std::ptr::null());

            gl::CompileShader(self.id);

            gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let error = create_whitespace_cstring_with_len(len as usize);

            // Write shader log into error
            unsafe {
                gl::GetShaderInfoLog(
                    self.id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(())
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
