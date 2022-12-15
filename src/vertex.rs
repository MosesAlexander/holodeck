use crate::gl;
use crate::buffer::*;

struct VertexDescriptor {
    buffer: BufferDescriptor,
    vao_id: gl::types::GLuint,
    ebo_id: gl::types::GLuint,
    format: gl::types::GLenum,
}

/*
 * component_groups: Groups of components vertices are made out of
 * component_nums:   Number of components for each group, i.e. 3 position, 3 color, 2 texture,
 */
struct AtrributesDescriptor {
    component_groups: gl::types::GLuint,
    components_nums: Vec<gl::types::GLint>,
    component_types: Vec<gl::types::GLenum>,
    component_offsets: Vec<usize>,
    component_strides: Vec<gl::types::GLint>,
}

impl VertexDescriptor {
    pub fn new(buffer: BufferDescriptor) -> VertexDescriptor {
        let mut vao_id = 0;
        let mut ebo_id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
            gl::GenBuffers(1, &mut ebo_id);
        }
        let vertex_descriptor = VertexDescriptor { buffer: buffer, vao_id: vao_id, ebo_id: ebo_id, format: gl::FLOAT };
        vertex_descriptor.buffer.bind();
        vertex_descriptor.bind();
        vertex_descriptor
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao_id);
        }
    }

    pub fn set_attributes(&mut self, attributes: AtrributesDescriptor) -> Result<(), String> {
        self.bind();

        for attr_idx in 0..attributes.component_groups {
            unsafe {
                gl::VertexAttribPointer(attr_idx, attributes.components_nums[attr_idx as usize],
                                        attributes.component_types[attr_idx as usize],
                                        gl::FALSE,
                                        match attributes.component_types[attr_idx as usize] {
                                            gl::FLOAT => (attributes.component_strides[attr_idx as usize] as usize * std::mem::size_of::<f32>()) as gl::types::GLint,
                                            gl::UNSIGNED_INT => (attributes.component_strides[attr_idx as usize] as usize * std::mem::size_of::<u32>()) as gl::types::GLint,
                                            _ => {
                                                return Err(format!("Invalid component type! {}", attributes.component_types[attr_idx as usize]));
                                            }
                                        },
                                        (attributes.component_offsets[attr_idx as usize] * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
                );
                gl::EnableVertexAttribArray(attr_idx);
            }
        }
        Ok(())
    }

    pub fn set_indexed_drawing(&mut self, indices_array: Vec<u32>) {
        self.bind();
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo_id);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                                    (indices_array.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                                    indices_array.as_ptr() as *const gl::types::GLvoid,
                                    gl::STATIC_DRAW
            )
        }
    }
}

