pub mod application;
pub mod buffer;
pub mod cube;
pub mod program;
pub mod shader;
pub mod texture;
pub mod uniform;
pub mod vertex;
pub mod quad;
pub mod text;

use application::{Application, FRAGMENT_SHADER, VERTEX_SHADER};
use buffer::BufferDescriptor;
use cube::*;
use program::Program;
use shader::Shader;
use texture::TextureDescriptor;
use uniform::*;
use vertex::{AttributesDescriptor, VertexDescriptor, Mesh, Model};
use quad::*;
use text::TextManager;

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
    let cube_attr = AttributesDescriptor {
        component_groups: 2,
        component_nums: vec![3, 2],
        component_types: vec![gl::FLOAT, gl::FLOAT, gl::FLOAT],
        component_offsets: vec![0, 3],
        component_strides: vec![5, 5],
    };
    let mut cube_mesh = Mesh::new(cube.vertices, cube.indices, cube_attr);

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

    cube_mesh.add_uniform(rotate_about_x_uniform);
    cube_mesh.add_uniform(rotate_about_y_uniform);
    cube_mesh.add_uniform(rotate_about_z_uniform);
    cube_mesh.add_uniform(translate_uniform);
    cube_mesh.add_uniform(mixvalue_uniform);
    cube_mesh.add_uniform(projection_uniform);
    cube_mesh.add_uniform(camera_uniform);

    cube_mesh.add_texture(texture1_desc);
    cube_mesh.add_texture(texture2_desc);

    let mut cube_model = Model::new();
    cube_model.add_mesh(cube_mesh);
    cube_model.attach_program(program_cube);

    let mut vert_shader_floor = Shader::new("src/floor.vert", VERTEX_SHADER);
    let mut frag_shader_floor = Shader::new("src/floor.frag", FRAGMENT_SHADER);

    match vert_shader_floor.compile() {
        Ok(()) => {}
        Err(e) => {
            println!("ERROR: {}, exiting program", e);
            std::process::exit(1);
        }
    }

    match frag_shader_floor.compile() {
        Ok(()) => {}
        Err(e) => {
            println!("ERROR: {}, exiting program", e);
            std::process::exit(1);
        }
    }

    let mut program_floor = Program::new();

    program_floor.add_shader(&vert_shader_floor);
    program_floor.add_shader(&frag_shader_floor);
    match program_floor.link_shaders() {
        Ok(()) => {}
        Err(e) => {
            println!("ERROR: {}, exiting program", e);
            std::process::exit(1);
        }
    }

    app.add_program(&program_floor);

    let floor = Quad::new(10.0, 0.0, (0.0, 0.000001, 0.0), (0.0,0.0,0.0), (10.0, 10.0));
    let floor_attr = AttributesDescriptor {
        component_groups: 2,
        component_nums: vec![3, 2],
        component_types: vec![gl::FLOAT, gl::FLOAT, gl::FLOAT],
        component_offsets: vec![0, 3],
        component_strides: vec![5, 5],
    };
    let mut floor_mesh = Mesh::new(floor.vertices, floor.indices, floor_attr);

    let floor_texture_desc =
        TextureDescriptor::new(program_floor.id, "texture1", "src/concrete_floor.jpg", gl::RGB);
    
    let projection_uniform = UniformDescriptor::new(program_floor.id, "projection");
    let camera_uniform = UniformDescriptor::new(program_floor.id, "look_at");

    floor_mesh.add_uniform(projection_uniform);
    floor_mesh.add_uniform(camera_uniform);

    floor_mesh.add_texture(floor_texture_desc);

    let mut floor_model = Model::new();
    floor_model.add_mesh(floor_mesh);
    floor_model.attach_program(program_floor);

    app.add_model(floor_model);

    let mut vert_shader_text = Shader::new("src/text.vert", VERTEX_SHADER);
    let mut frag_shader_text = Shader::new("src/text.frag", FRAGMENT_SHADER);

    match vert_shader_text.compile() {
        Ok(()) => {}
        Err(e) => {
            println!("ERROR: {}, exiting program", e);
            std::process::exit(1);
        }
    }

    match frag_shader_text.compile() {
        Ok(()) => {}
        Err(e) => {
            println!("ERROR: {}, exiting program", e);
            std::process::exit(1);
        }
    }

    let mut program_text = Program::new();

    program_text.add_shader(&vert_shader_text);
    program_text.add_shader(&frag_shader_text);
    match program_text.link_shaders() {
        Ok(()) => {}
        Err(e) => {
            println!("ERROR: {}, exiting program", e);
            std::process::exit(1);
        }
    }

    let text_manager = TextManager::new(program_text);

    app.attach_text_manager(text_manager);

    let wall1 = Quad::new(2.5, 0.6, (0.0, 0.0, 2.5), (0.0,0.0,2.5), (2.0, 1.0));
    let wall1_buffer = BufferDescriptor::new(&wall1.vertices);
    let mut wall1_vert_desc = VertexDescriptor::new(wall1_buffer);
    let wall1_attr = AttributesDescriptor {
        component_groups: 2,
        component_nums: vec![3, 2],
        component_types: vec![gl::FLOAT, gl::FLOAT, gl::FLOAT],
        component_offsets: vec![0, 3],
        component_strides: vec![5, 5],
    };
    wall1_vert_desc.set_attributes(wall1_attr);

    let wall1_texture_desc =
        TextureDescriptor::new(program_floor.id, "texture1", "src/brick_wall.jpg", gl::RGB);
    
    let projection_uniform = UniformDescriptor::new(program_floor.id, "projection");
    let camera_uniform = UniformDescriptor::new(program_floor.id, "look_at");

    wall1_vert_desc.add_uniform(projection_uniform);
    wall1_vert_desc.add_uniform(camera_uniform);

    wall1_vert_desc.add_texture(wall1_texture_desc);

    wall1_vert_desc.set_indexed_drawing(wall1.indices);

    app.add_vertex_descriptor(wall1_vert_desc);

    let wall2 = Quad::new(2.5, 0.6, (0.0, 0.0, -2.5), (0.0,0.0,-2.5), (2.0, 1.0));
    let wall2_buffer = BufferDescriptor::new(&wall2.vertices);
    let mut wall2_vert_desc = VertexDescriptor::new(wall2_buffer);
    let wall2_attr = AttributesDescriptor {
        component_groups: 2,
        component_nums: vec![3, 2],
        component_types: vec![gl::FLOAT, gl::FLOAT, gl::FLOAT],
        component_offsets: vec![0, 3],
        component_strides: vec![5, 5],
    };
    wall2_vert_desc.set_attributes(wall2_attr);

    let wall2_texture_desc =
        TextureDescriptor::new(program_floor.id, "texture1", "src/brick_wall.jpg", gl::RGB);
    
    let projection_uniform = UniformDescriptor::new(program_floor.id, "projection");
    let camera_uniform = UniformDescriptor::new(program_floor.id, "look_at");

    wall2_vert_desc.add_uniform(projection_uniform);
    wall2_vert_desc.add_uniform(camera_uniform);

    wall2_vert_desc.add_texture(wall2_texture_desc);

    wall2_vert_desc.set_indexed_drawing(wall2.indices);

    app.add_vertex_descriptor(wall2_vert_desc);

    let wall3 = Quad::new(5.0, 0.6, (1.25, 0.0, 0.0), (1.25,0.0,0.0), (2.0, 1.0));
    let wall3_buffer = BufferDescriptor::new(&wall3.vertices);
    let mut wall3_vert_desc = VertexDescriptor::new(wall3_buffer);
    let wall3_attr = AttributesDescriptor {
        component_groups: 2,
        component_nums: vec![3, 2],
        component_types: vec![gl::FLOAT, gl::FLOAT, gl::FLOAT],
        component_offsets: vec![0, 3],
        component_strides: vec![5, 5],
    };
    wall3_vert_desc.set_attributes(wall3_attr);

    let wall3_texture_desc =
        TextureDescriptor::new(program_floor.id, "texture1", "src/brick_wall.jpg", gl::RGB);
    
    let projection_uniform = UniformDescriptor::new(program_floor.id, "projection");
    let camera_uniform = UniformDescriptor::new(program_floor.id, "look_at");

    wall3_vert_desc.add_uniform(projection_uniform);
    wall3_vert_desc.add_uniform(camera_uniform);

    wall3_vert_desc.add_texture(wall3_texture_desc);

    wall3_vert_desc.set_indexed_drawing(wall3.indices);

    app.add_vertex_descriptor(wall3_vert_desc);

    let wall4 = Quad::new(5.0, 0.6, (-1.25, 0.0, 0.0), (-1.25,0.0,0.0), (2.0,1.0));
    let wall4_buffer = BufferDescriptor::new(&wall4.vertices);
    let mut wall4_vert_desc = VertexDescriptor::new(wall4_buffer);
    let wall4_attr = AttributesDescriptor {
        component_groups: 2,
        component_nums: vec![3, 2],
        component_types: vec![gl::FLOAT, gl::FLOAT, gl::FLOAT],
        component_offsets: vec![0, 3],
        component_strides: vec![5, 5],
    };
    wall4_vert_desc.set_attributes(wall4_attr);

    let wall4_texture_desc =
        TextureDescriptor::new(program_floor.id, "texture1", "src/brick_wall.jpg", gl::RGB);
    
    let projection_uniform = UniformDescriptor::new(program_floor.id, "projection");
    let camera_uniform = UniformDescriptor::new(program_floor.id, "look_at");

    wall4_vert_desc.add_uniform(projection_uniform);
    wall4_vert_desc.add_uniform(camera_uniform);

    wall4_vert_desc.add_texture(wall4_texture_desc);

    wall4_vert_desc.set_indexed_drawing(wall4.indices);

    app.add_vertex_descriptor(wall4_vert_desc);


    app.render_vaos();
}
