use crate::gl;
use stb_image::stb_image::bindgen::*;
use std::ffi::{c_int, c_void, CString};

pub struct TextureDescriptor {
    texture_id: gl::types::GLuint,
    texture_shader_handle: gl::types::GLint,
}

impl TextureDescriptor {
    pub fn new(
        bound_program_id: gl::types::GLuint,
        shader_handle_name: &str,
        path: &str,
        format: gl::types::GLenum,
    ) -> TextureDescriptor {
        // Texture generation part
        let mut width: c_int = 0;
        let mut height: c_int = 0;
        let mut nr_channels: c_int = 0;
        let mut texture_id: gl::types::GLuint = 0;
        let path_string = CString::new(path).unwrap();
        let mut texture_shader_handle = 0;
        unsafe {
            texture_shader_handle = gl::GetUniformLocation(
                bound_program_id,
                CString::new(shader_handle_name.to_string())
                    .unwrap()
                    .as_ptr(),
            );
            gl::GenTextures(1, &mut texture_id);
        }
        let texture_desc = TextureDescriptor {
            texture_id: texture_id,
            texture_shader_handle: texture_shader_handle,
        };

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // Set the texture wrapping/filtering options on the currently bound texture object
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            stbi_set_flip_vertically_on_load(1);
            let buffer = stbi_load(
                path_string.as_ptr(),
                &mut width,
                &mut height,
                &mut nr_channels,
                0,
            );

            if !buffer.is_null() {
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGB as i32,
                    width,
                    height,
                    0,
                    format,
                    gl::UNSIGNED_BYTE,
                    buffer as *const c_void,
                );
                gl::GenerateMipmap(gl::TEXTURE_2D);
                stbi_image_free(buffer as *mut c_void);
            } else {
                println!("Failed to load texture!");
            }
        }
        texture_desc
    }

    pub fn set_active_texture(&self, idx: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + idx);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
            gl::Uniform1i(self.texture_shader_handle as i32, idx as i32);
        }
    }
}
