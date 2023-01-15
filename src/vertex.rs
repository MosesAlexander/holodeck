use std::rc::Rc;

use crate::buffer::*;
use crate::gl;
use crate::program::Program;
use crate::texture::TextureDescriptor;
use crate::uniform::UniformDescriptor;
use glam::*;

// Each model can have several sub-models/shapes
// that it consists of
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub program: Option<Rc<Program>>,
}

// Models can be made up of multiple meshes
impl Model {
    pub fn new() -> Model {
        Model { meshes: Vec::new(), program: None }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }

    pub fn parse_mesh_from_file() {

    }

    pub fn attach_program(&mut self, program: Rc<Program>) {
        self.program = Some(program);
    }

    pub fn render(&self) {
        for mesh in self.meshes.iter() {
            unsafe {
                gl::DrawElements(
                    gl::TRIANGLES,
                    mesh.ebo.num_ebo_elements as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
            }
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.program.as_ref().unwrap().id);
        }
    }
}

// Represents a basic shape a model is made of
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub face_indices: Rc<Vec<u32>>,
    pub textures: Vec<TextureDescriptor>,
    pub uniforms: Vec<UniformDescriptor>,
    buffer: Rc<BufferDescriptor>,
    vao: VaoDescriptor,
    ebo: Rc<EboDescriptor>,
}

impl Mesh {
    pub fn new(vertices: Vec<f32>, indices: Vec<u32>, attributes: AttributesDescriptor) -> Mesh {
        let indices_ref = Rc::new(indices);
        let buffer = Rc::new(BufferDescriptor::new(&vertices));
        let mut vao = VaoDescriptor::new(attributes, Rc::clone(&buffer));
        let ebo = Rc::new(EboDescriptor::new(Rc::clone(&indices_ref)));
        vao.attach_ebo(Rc::clone(&ebo));

        Mesh {
            buffer: buffer,
            vertices: vertices,
            face_indices: indices_ref,
            textures: Vec::new(),
            uniforms: Vec::new(),
            vao: vao,
            ebo: ebo,
        }
    }

    pub fn bind_vao(&self) {
        self.vao.bind();
    }

    pub fn add_uniform(&mut self, uniform: UniformDescriptor) {
        self.uniforms.push(uniform);
    }

    pub fn add_texture(&mut self, texture: TextureDescriptor) {
        self.textures.push(texture);
    }

    pub fn render(&self) {
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.ebo.num_ebo_elements as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}

/*
 * component_groups: Groups of components vertices are made out of
 * component_nums:   Number of components for each group, i.e. 3 position, 3 color, 2 texture,
 */
pub struct AttributesDescriptor {
    pub component_groups: gl::types::GLuint,
    pub component_nums: Vec<gl::types::GLint>,
    pub component_types: Vec<gl::types::GLenum>,
    pub component_offsets: Vec<usize>,
    pub component_strides: Vec<gl::types::GLint>,
}
