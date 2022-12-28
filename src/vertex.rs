use crate::buffer::*;
use crate::gl;
use crate::texture::TextureDescriptor;
use crate::uniform::UniformDescriptor;

pub struct VertexDescriptor {
    buffer: BufferDescriptor,
    vao_id: gl::types::GLuint,
    ebo_id: gl::types::GLuint,
    format: gl::types::GLenum,
    pub uniforms: Vec<UniformDescriptor>,
    pub textures: Vec<TextureDescriptor>,
    num_elements: gl::types::GLsizei,
}

/*
 * component_groups: Groups of components vertices are made out of
 * component_nums:   Number of components for each group, i.e. 3 position, 3 color, 2 texture,
 */
pub struct AtrributesDescriptor {
    pub component_groups: gl::types::GLuint,
    pub component_nums: Vec<gl::types::GLint>,
    pub component_types: Vec<gl::types::GLenum>,
    pub component_offsets: Vec<usize>,
    pub component_strides: Vec<gl::types::GLint>,
}

impl VertexDescriptor {
    pub fn new(buffer: BufferDescriptor) -> VertexDescriptor {
        let mut vao_id = 0;
        let mut ebo_id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
            gl::GenBuffers(1, &mut ebo_id);
        }

        let vertex_descriptor = VertexDescriptor {
            buffer: buffer,
            vao_id: vao_id,
            ebo_id: ebo_id,
            format: gl::FLOAT,
            uniforms: Vec::new(),
            textures: Vec::new(),
            num_elements: 0,
        };

        //vertex_descriptor.buffer.bind();
        //vertex_descriptor.bind();
        vertex_descriptor
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao_id);
        }
    }

    pub fn render(&self) {
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.num_elements,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }

    pub fn set_attributes(&mut self, attributes: AtrributesDescriptor) -> Result<(), String> {
        self.bind();
        self.buffer.bind();

        for attr_idx in 0..attributes.component_groups {
            unsafe {
                gl::VertexAttribPointer(
                    attr_idx,
                    attributes.component_nums[attr_idx as usize],
                    attributes.component_types[attr_idx as usize],
                    gl::FALSE,
                    match attributes.component_types[attr_idx as usize] {
                        gl::FLOAT => {
                            (attributes.component_strides[attr_idx as usize] as usize
                                * std::mem::size_of::<f32>())
                                as gl::types::GLint
                        }
                        gl::UNSIGNED_INT => {
                            (attributes.component_strides[attr_idx as usize] as usize
                                * std::mem::size_of::<u32>())
                                as gl::types::GLint
                        }
                        _ => {
                            return Err(format!(
                                "Invalid component type! {}",
                                attributes.component_types[attr_idx as usize]
                            ));
                        }
                    },
                    (attributes.component_offsets[attr_idx as usize] * std::mem::size_of::<f32>())
                        as *const gl::types::GLvoid,
                );
                gl::EnableVertexAttribArray(attr_idx);
            }
        }
        Ok(())
    }

    pub fn set_indexed_drawing(&mut self, indices_array: Vec<u32>) {
        self.bind();
        self.num_elements = indices_array.len() as i32;
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo_id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices_array.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices_array.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            )
        }
    }

    pub fn add_uniform(&mut self, uniform: UniformDescriptor) {
        self.uniforms.push(uniform);
    }

    pub fn add_texture(&mut self, texture: TextureDescriptor) {
        self.textures.push(texture);
    }
}
