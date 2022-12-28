use crate::shader::*;

use std::ffi::CString;

use crate::gl;

pub struct Program {
    pub id: gl::types::GLuint,
    shader_ids: Vec<gl::types::GLuint>,
}

impl Program {
    pub fn new() -> Program {
        unsafe {
            Program {
                id: gl::CreateProgram(),
                shader_ids: Vec::new(),
            }
        }
    }

    pub fn add_shader(&mut self, shader: &Shader) {
        self.shader_ids.push(shader.id);
    }

    pub fn link_shaders(&self) -> Result<(), String> {
        for shader in self.shader_ids.iter() {
            unsafe {
                gl::AttachShader(self.id, *shader);
            }
        }

        unsafe {
            gl::LinkProgram(self.id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success);

            let mut len: gl::types::GLint = 0;
            if success == 0 {
                gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut len);

                let error = create_whitespace_cstring_with_len(len as usize);

                gl::GetProgramInfoLog(
                    self.id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );

                return Err(error.to_string_lossy().into_owned());
            }
        }

        for shader in self.shader_ids.iter() {
            unsafe {
                gl::DetachShader(self.id, *shader);
            }
        }

        Ok(())
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
