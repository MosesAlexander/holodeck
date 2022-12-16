use crate::gl;
use std::ffi::{CString, c_int, c_void};
use stb_image::stb_image::bindgen::*;

pub struct TextureDescriptor {
    texture_id: gl::types::GLuint
}

impl TextureDescriptor {
    pub fn new(path: &str) -> TextureDescriptor {
        // Texture generation part
        let mut width: c_int = 0;
        let mut height: c_int = 0;
        let mut nr_channels: c_int = 0;
        let mut texture_id: gl::types::GLuint = 0;
        let mut path_string = CString::new(path).unwrap();

        unsafe {

            gl::GenTextures(1, &mut texture_id);
        }
        let texture_desc = TextureDescriptor { texture_id: texture_id};
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // Set the texture wrapping/filtering options on the currently bound texture object
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            stbi_set_flip_vertically_on_load(1);
            let buffer = stbi_load(path_string.as_ptr(), &mut width, &mut height, &mut nr_channels, 0);

            if (!buffer.is_null()) {
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, width, height, 0,
                                gl::RGB, gl::UNSIGNED_BYTE, buffer as *const c_void);
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
            gl::Uniform1i(self.texture_id as i32, idx as i32);
        }
    }

}