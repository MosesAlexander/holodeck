pub mod application;
pub mod buffer;
pub mod cube;
pub mod program;
pub mod shader;
pub mod texture;
pub mod uniform;
pub mod vertex;

use application::{Application, FRAGMENT_SHADER, VERTEX_SHADER};
use buffer::BufferDescriptor;
use cube::*;
use program::Program;
use shader::Shader;
use texture::TextureDescriptor;
use uniform::*;
use vertex::{AtrributesDescriptor, VertexDescriptor};

mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

fn main() {
    let mut app = Application::new();

    let mut vert_shader_cube = Shader::new("src/cube.vert", VERTEX_SHADER);
    let mut frag_shader_cube = Shader::new("src/cube.frag", FRAGMENT_SHADER);

    match vert_shader_cube.compile() {
        Ok(()) => {}
        Err(e) => {
            println!("ERROR: {}, exiting program", e);
            std::process::exit(1);
        }
    }

    match frag_shader_cube.compile() {
        Ok(()) => {}
        Err(e) => {
            println!("ERROR: {}, exiting program", e);
            std::process::exit(1);
        }
    }

    let mut program_cube = Program::new();

    program_cube.add_shader(&vert_shader_cube);
    program_cube.add_shader(&frag_shader_cube);
    match program_cube.link_shaders() {
        Ok(()) => {}
        Err(e) => {
            println!("ERROR: {}, exiting program", e);
            std::process::exit(1);
        }
    }

    app.add_program(&program_cube);

    let cube = Cube::new(0.1, (0.0, 0.0, 0.0));
    let buffer2 = BufferDescriptor::new(cube.vertices, cube.center);
    let mut cube_vert_desc = VertexDescriptor::new(buffer2);
    let cube_attr = AtrributesDescriptor {
        component_groups: 2,
        component_nums: vec![3, 2],
        component_types: vec![gl::FLOAT, gl::FLOAT, gl::FLOAT],
        component_offsets: vec![0, 3],
        component_strides: vec![5, 5],
    };
    cube_vert_desc.set_attributes(cube_attr);

    let texture1_desc =
        TextureDescriptor::new(program_cube.id, "texture1", "src/stallman.jpg", gl::RGB);
    let texture2_desc =
		TextureDescriptor::new(program_cube.id, "texture2", "src/gnu.png", gl::RGBA);

    let rotate_about_x_uniform = UniformDescriptor::new(program_cube.id, "rotate_about_x");

    let rotate_about_y_uniform = UniformDescriptor::new(program_cube.id, "rotate_about_y");

    let rotate_about_z_uniform = UniformDescriptor::new(program_cube.id, "rotate_about_z");

    let translate_uniform = UniformDescriptor::new(program_cube.id, "translate");

    let mixvalue_uniform = UniformDescriptor::new(program_cube.id, "mixvalue");

    let projection_uniform = UniformDescriptor::new(program_cube.id, "projection");

    let camera_uniform = UniformDescriptor::new(program_cube.id, "look_at");

    cube_vert_desc.add_uniform(rotate_about_x_uniform);
    cube_vert_desc.add_uniform(rotate_about_y_uniform);
    cube_vert_desc.add_uniform(rotate_about_z_uniform);
    cube_vert_desc.add_uniform(translate_uniform);
    cube_vert_desc.add_uniform(mixvalue_uniform);
    cube_vert_desc.add_uniform(projection_uniform);
    cube_vert_desc.add_uniform(camera_uniform);

    cube_vert_desc.add_texture(texture1_desc);
    cube_vert_desc.add_texture(texture2_desc);
    cube_vert_desc.set_indexed_drawing(cube.indices);

    app.add_vertex_descriptor(cube_vert_desc);

    app.render_vaos();
}
