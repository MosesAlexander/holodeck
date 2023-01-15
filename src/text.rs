use std::collections::HashMap;
use std::ffi::{CString, c_void};

use freetype::freetype::{FT_Done_FreeType, FT_Library, FT_Face, FT_Init_FreeType, FT_New_Face, FT_Set_Pixel_Sizes, FT_LOAD_RENDER, FT_Load_Char, FT_Done_Face};
use glam::{Mat4, IVec2, Vec3};
use crate::gl::types::{GLint, GLuint};
use crate::gl::{self, ARRAY_BUFFER};

use crate::uniform::{UniformPackedParam, Uniform3FParam};
use crate::{program::Program, uniform::{UniformDescriptor, Uniform4FVMatrix}};

pub struct TextManager {
    program: Program,
    text_uniform: UniformDescriptor,
    text_projection_uniform: UniformDescriptor,
    text_projection: Mat4,
    characters: HashMap<char, Character>,
    text_vao: gl::types::GLuint,
    text_vbo: gl::types::GLuint,
}

pub struct Character {
    TextureID: u32,
    Size: IVec2,
    Bearing: IVec2,
    Advance: u32,
}

impl TextManager {
    pub fn new(program: Program) -> TextManager {
        let mut text_uniform = UniformDescriptor::new(
            program.id,
            "textColor"
        );

        let text_projection: Mat4 = Mat4::orthographic_rh_gl(0.0, 1024.0, 0.0, 768.0, -1.0, 1.0);

        let mut text_proj_uniform = UniformDescriptor::new(
            program.id,
            "projection"
        );

        let mut text_vbo = 0;
        let mut text_vao = 0;

        // For now we reserve enough memory when initiating the VBO so that we can later update the VBO's memory
        // when rendering characters:
        unsafe {
            gl::GenVertexArrays(1, &mut text_vao);
            gl::GenBuffers(1, &mut text_vbo);
            gl::BindVertexArray(text_vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, text_vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                (std::mem::size_of::<f32>() * 6 * 4) as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                4,
                gl::FLOAT,
                gl::FALSE,
                (4 * std::mem::size_of::<f32>()) as i32,
                0 as *const gl::types::GLvoid
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        TextManager { program: program,
                      text_uniform: text_uniform,
                      characters: HashMap::new(),
                      text_projection_uniform: text_proj_uniform,
                      text_projection: text_projection,
                      text_vao: text_vao,
                      text_vbo: text_vbo}
    }

    pub fn init(&mut self) {
        let mut ft: FT_Library = std::ptr::null_mut();
        let mut face: FT_Face = std::ptr::null_mut();

        unsafe {
            let mut ret = FT_Init_FreeType(&mut ft as *mut FT_Library);
            if ret != 0 {
                eprintln!("ERROR_FREETYPE: Failed initializing FreeType library!");
                std::process::exit(1);
            }

            {
                let font_path = CString::new("res/Hack-Regular.ttf").unwrap(); 
                ret = FT_New_Face(ft,
                    font_path.as_ptr(),
                    0,
                    &mut face as *mut FT_Face
                );
                if ret != 0 {
                    eprintln!("ERROR::FREETYPE: Failed to load font");
                    std::process::exit(1);
                }
            }

            ret = FT_Set_Pixel_Sizes(face, 0, 48);
            if ret != 0 {
                eprintln!("ERROR::FREETYPE: Error setting font size");
                std::process::exit(1);
            }
        }

        // disable byte alignment restriction
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }

        unsafe {
            let mut ret;

            for c in 0..128u8 {
                ret = FT_Load_Char(face, c as u64, FT_LOAD_RENDER as i32);
                if ret != 0 {
                    eprintln!("ERROR::FREETYPE: Error loading character");
                    std::process::exit(1);
                }

                let mut texture: GLuint = 0;
                gl::GenTextures(1, &mut texture as *mut u32);
                gl::BindTexture(gl::TEXTURE_2D, texture);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RED as i32,
                    (*(*face).glyph).bitmap.width as i32,
                   (*(*face).glyph).bitmap.rows as i32,
                    0,
                    gl::RED,
                    gl::UNSIGNED_BYTE,
                    (*(*face).glyph).bitmap.buffer as *const c_void
                );

                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

                let character = Character {
                    TextureID: texture,
                    Size: IVec2::new((*(*face).glyph).bitmap.width as i32, (*(*face).glyph).bitmap.rows as i32),
                    Bearing: IVec2::new((*(*face).glyph).bitmap_left,  (*(*face).glyph).bitmap_top),
                    Advance: (*(*face).glyph).advance.x as u32
                };

                self.characters.insert(c as char, character);
            }

            ret = FT_Done_Face(face);
            if ret != 0 {
                eprintln!("ERROR::FREETYPE: Error freeing Face resources");
                std::process::exit(1);
            }
            ret = FT_Done_FreeType(ft);
            if ret != 0 {
                eprintln!("ERROR::FREETYPE: Error freeing FreeType resources");
                std::process::exit(1);
            }
        }

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }


        self.use_text_program();

        self.text_projection_uniform.update(UniformPackedParam::UniformMatrix4FV(
            Uniform4FVMatrix(self.text_projection),
        ));

        
    }

    pub fn use_text_program(&self) {
        unsafe {
            gl::UseProgram(self.program.id);
        }
    }

    pub fn render_text(&mut self, text: String, mut x: f32, mut y: f32, scale: f32, color: Vec3) {
        self.text_uniform.update(
            UniformPackedParam::Uniform3F(
                    Uniform3FParam(color.x, color.y, color.z)
            )
        );
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindVertexArray(self.text_vao);
        }

        for c in text.chars() {
            let ch = self.characters.get(&c).unwrap();

            let xpos = x + ch.Bearing.x as f32 * scale;
            let ypos = y - (ch.Size.y - ch.Bearing.y) as f32 * scale;

            let w = ch.Size.x as f32 * scale;
            let h = ch.Size.y as f32 * scale;
            // Update VBO for each character
            let mut vertices: Vec<f32> = Vec::with_capacity(6 * 4);
            vertices.extend_from_slice(&[xpos,     ypos + h, 0.0, 0.0]);
            vertices.extend_from_slice(&[xpos,     ypos,     0.0, 1.0]);
            vertices.extend_from_slice(&[xpos + w, ypos,     1.0, 1.0]);

            vertices.extend_from_slice(&[xpos,     ypos + h, 0.0, 0.0]);
            vertices.extend_from_slice(&[xpos + w, ypos,     1.0, 1.0]);
            vertices.extend_from_slice(&[xpos + w, ypos + h, 1.0, 0.0]);

            unsafe {
                // render glyph texture over quad
                gl::BindTexture(gl::TEXTURE_2D, ch.TextureID);
                // Update the content of the VBO memory
                gl::BindBuffer(gl::ARRAY_BUFFER, self.text_vbo);
                gl::BufferSubData(gl::ARRAY_BUFFER,
                    0,
                    (vertices.len() * std::mem::size_of::<f32>()) as isize,
                    vertices.as_ptr() as *const c_void
                );
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                // render quad
                gl::DrawArrays(gl::TRIANGLES, 0, 6);
                // now advance cursors for next glyph(note that advance is number of 1/64 pixels)
                x += (ch.Advance >> 6) as f32 * scale; // bitshift by 6 to get value in pixels (2^6 = 64)
            }
        }

        unsafe {
            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

}