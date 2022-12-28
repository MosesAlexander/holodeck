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

    let mut vert_shader = Shader::new("src/triangle.vert", VERTEX_SHADER);
    let mut frag_shader1 = Shader::new("src/triangle3.frag", FRAGMENT_SHADER);
    let mut frag_shader2 = Shader::new("src/triangle2.frag", FRAGMENT_SHADER);

    match vert_shader.compile() {
        Ok(()) => {}
        Err(e) => {
            println!("ERROR: {}, exiting program", e);
            std::process::exit(1);
        }
    }

    match frag_shader1.compile() {
        Ok(()) => {}
        Err(e) => {
            println!("ERROR: {}, exiting program", e);
            std::process::exit(1);
        }
    }

    match frag_shader2.compile() {
        Ok(()) => {}
        Err(e) => {
            println!("ERROR: {}, exiting program", e);
            std::process::exit(1);
        }
    }

    let mut program1 = Program::new();
    let mut program2 = Program::new();

    program1.add_shader(&vert_shader);
    program1.add_shader(&frag_shader1);
    match program1.link_shaders() {
        Ok(()) => {}
        Err(e) => {
            println!("ERROR: {}, exiting program", e);
            std::process::exit(1);
        }
    }

    program2.add_shader(&vert_shader);
    program2.add_shader(&frag_shader2);
    match program2.link_shaders() {
        Ok(()) => {}
        Err(e) => {
            println!("ERROR: {}, exiting program", e);
            std::process::exit(1);
        }
    }

    app.add_program(&program1);
    app.add_program(&program2);

    let vertices_indexed_two_triangles: Vec<f32> = vec![
        0.0, 0.0, 0.98, 0.0, 1.0, 0.0, -0.5, 0.0, 0.98, 1.0, 00.0, 0.0, -0.25, 0.5, 0.98, 0.0, 0.0,
        1.0, 0.25, 0.5, 0.98, 0.0, 1.0, 0.0, 0.5, 0.0, 0.98, 0.0, 0.0, 1.0,
    ];

    let indices_two_triangles: Vec<u32> = vec![0, 1, 2, 2, 0, 3];

    let cube = Cube::new(0.1, (0.0, 0.0, 0.0));

    let mut buffer1 = BufferDescriptor::new(vertices_indexed_two_triangles, (0.0, 0.0, 0.0));
    let mut two_triangles_vert_desc = VertexDescriptor::new(buffer1);
    let mut two_triangles_attr = AtrributesDescriptor {
        component_groups: 2,
        component_nums: vec![3, 3],
        component_types: vec![gl::FLOAT, gl::FLOAT],
        component_offsets: vec![0, 3],
        component_strides: vec![6, 6],
    };

    match two_triangles_vert_desc.set_attributes(two_triangles_attr) {
        Ok(()) => {}
        Err(e) => {
            println!("ERROR setting attributes: {}", e);
            std::process::exit(1);
        }
    }
    two_triangles_vert_desc.set_indexed_drawing(indices_two_triangles);

    let color1_uniform = UniformDescriptor::new(program1.id, "color1");
    two_triangles_vert_desc.add_uniform(color1_uniform);

    let color2_uniform = UniformDescriptor::new(program1.id, "color2");
    two_triangles_vert_desc.add_uniform(color2_uniform);

    let color3_uniform = UniformDescriptor::new(program1.id, "color3");
    two_triangles_vert_desc.add_uniform(color3_uniform);

    let color4_uniform = UniformDescriptor::new(program1.id, "color4");
    two_triangles_vert_desc.add_uniform(color4_uniform);

    let mut buffer2 = BufferDescriptor::new(cube.vertices, cube.center);
    let mut cube_vert_desc = VertexDescriptor::new(buffer2);
    let mut cube_attr = AtrributesDescriptor {
        component_groups: 3,
        component_nums: vec![3, 3, 2],
        component_types: vec![gl::FLOAT, gl::FLOAT, gl::FLOAT],
        component_offsets: vec![0, 3, 6],
        component_strides: vec![8, 8, 8],
    };
    cube_vert_desc.set_attributes(cube_attr);

    let texture1_desc =
        TextureDescriptor::new(program2.id, "texture1", "src/stallman.jpg", gl::RGB);
    let texture2_desc = TextureDescriptor::new(program2.id, "texture2", "src/gnu.png", gl::RGBA);

    let rotate_about_x_uniform = UniformDescriptor::new(program2.id, "rotate_about_x");

    let rotate_about_y_uniform = UniformDescriptor::new(program2.id, "rotate_about_y");

    let rotate_about_z_uniform = UniformDescriptor::new(program2.id, "rotate_about_z");

    let translate_uniform = UniformDescriptor::new(program2.id, "translate");

    let mixvalue_uniform = UniformDescriptor::new(program2.id, "mixvalue");

    let projection_uniform = UniformDescriptor::new(program2.id, "projection");

    cube_vert_desc.add_uniform(rotate_about_x_uniform);
    cube_vert_desc.add_uniform(rotate_about_y_uniform);
    cube_vert_desc.add_uniform(rotate_about_z_uniform);
    cube_vert_desc.add_uniform(translate_uniform);
    cube_vert_desc.add_uniform(mixvalue_uniform);
    cube_vert_desc.add_uniform(projection_uniform);

    cube_vert_desc.add_texture(texture1_desc);
    cube_vert_desc.add_texture(texture2_desc);
    cube_vert_desc.set_indexed_drawing(cube.indices);

    app.add_vertex_descriptor(two_triangles_vert_desc);
    app.add_vertex_descriptor(cube_vert_desc);

    app.render_vaos();
}
