use crate::gl;

pub struct BufferDescriptor {
    buffer_id: gl::types::GLuint,
    position_in_world: (f32,f32,f32),
}

impl BufferDescriptor {
    pub fn new(vertices: Vec<f32>, position_in_world: (f32,f32,f32)) -> BufferDescriptor {
        let mut buffer_id = 0;

        unsafe {
            gl::GenBuffers(1, &mut buffer_id);
        }
        let buffer = BufferDescriptor { buffer_id: buffer_id, position_in_world: position_in_world };


        buffer.bind();
        
        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
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