use std::rc::Rc;

use crate::gl;
use crate::vertex::*;

pub struct BufferDescriptor {
    buffer_id: gl::types::GLuint,
}

impl BufferDescriptor {
    pub fn new(vertices: &Vec<f32>) -> BufferDescriptor {
        let mut buffer_id = 0;

        unsafe {
            gl::GenBuffers(1, &mut buffer_id);
        }
        let buffer = BufferDescriptor {
            buffer_id: buffer_id,
        };

        buffer.bind();

        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }

        buffer
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer_id);
        }
    }
}

pub struct EboDescriptor {
    ebo_id: gl::types::GLuint,
    pub num_ebo_elements: u32,
    ebo_indices: Rc<Vec<u32>>,
}

impl EboDescriptor {
    pub fn new(indices: Rc<Vec<u32>>) -> EboDescriptor {
        let mut ebo_id = 0;
        unsafe {
            gl::GenBuffers(1, &mut ebo_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
        
        EboDescriptor { ebo_id: ebo_id , num_ebo_elements: indices.len() as u32, ebo_indices: indices }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo_id);
        }
    }
}

pub struct VaoDescriptor {
    vao_id: gl::types::GLuint,
    buffer_ref: Rc<BufferDescriptor>,
    ebo: Option<Rc<EboDescriptor>>,
}

impl VaoDescriptor {
    pub fn new(attr: AttributesDescriptor, buffer_ref: Rc<BufferDescriptor>) -> VaoDescriptor {
        let mut vao_id = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
        }

        VaoDescriptor { vao_id: vao_id, buffer_ref: buffer_ref, ebo: None}
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao_id);
        }
    }



    pub fn set_attributes(&mut self, attributes: AttributesDescriptor) -> Result<(), String> {
        self.bind();
        self.buffer_ref.bind();

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

    pub fn attach_ebo(&mut self, ebo: Rc<EboDescriptor>) {
        self.ebo = Some(ebo);
        self.buffer_ref.bind();
        self.bind();
        self.ebo.as_mut().unwrap().bind();
    }

}