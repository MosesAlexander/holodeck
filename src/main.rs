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

use std::rc::Rc;

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
    cube_model.attach_program(Rc::new(program_cube));
    app.add_model(cube_model);

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

    let program_floor_ref = Rc::new(program_floor);
    let program_wall1 = Rc::clone(&program_floor_ref);
    let program_wall2 = Rc::clone(&program_floor_ref);
    let program_wall3 = Rc::clone(&program_floor_ref);
    let program_wall4 = Rc::clone(&program_floor_ref);
    floor_model.attach_program(Rc::clone(&program_floor_ref));

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
    let wall1_attr = AttributesDescriptor {
        component_groups: 2,
        component_nums: vec![3, 2],
        component_types: vec![gl::FLOAT, gl::FLOAT, gl::FLOAT],
        component_offsets: vec![0, 3],
        component_strides: vec![5, 5],
    };
    let mut wall1_mesh = Mesh::new(wall1.vertices, wall1.indices, wall1_attr);

    let wall1_texture_desc =
        TextureDescriptor::new(program_wall1.id, "texture1", "src/brick_wall.jpg", gl::RGB);
    
    let projection_uniform = UniformDescriptor::new(program_wall1.id, "projection");
    let camera_uniform = UniformDescriptor::new(program_wall1.id, "look_at");

    wall1_mesh.add_uniform(projection_uniform);
    wall1_mesh.add_uniform(camera_uniform);

    wall1_mesh.add_texture(wall1_texture_desc);

    let mut wall1_model = Model::new();
    wall1_model.add_mesh(wall1_mesh);
    wall1_model.attach_program(program_wall1);

    app.add_model(wall1_model);


    let wall2 = Quad::new(2.5, 0.6, (0.0, 0.0, -2.5), (0.0,0.0,-2.5), (2.0, 1.0));
    let wall2_attr = AttributesDescriptor {
        component_groups: 2,
        component_nums: vec![3, 2],
        component_types: vec![gl::FLOAT, gl::FLOAT, gl::FLOAT],
        component_offsets: vec![0, 3],
        component_strides: vec![5, 5],
    };
    let mut wall2_mesh = Mesh::new(wall2.vertices, wall2.indices, wall2_attr);

    let wall2_texture_desc =
        TextureDescriptor::new(program_wall2.id, "texture1", "src/brick_wall.jpg", gl::RGB);
    
    let projection_uniform = UniformDescriptor::new(program_wall2.id, "projection");
    let camera_uniform = UniformDescriptor::new(program_wall2.id, "look_at");

    wall2_mesh.add_uniform(projection_uniform);
    wall2_mesh.add_uniform(camera_uniform);

    wall2_mesh.add_texture(wall2_texture_desc);

    let mut wall2_model = Model::new();
    wall2_model.add_mesh(wall2_mesh);
    wall2_model.attach_program(program_wall2);

    app.add_model(wall2_model);

    let wall3 = Quad::new(5.0, 0.6, (1.25, 0.0, 0.0), (1.25,0.0,0.0), (2.0, 1.0));
    let wall3_attr = AttributesDescriptor {
        component_groups: 2,
        component_nums: vec![3, 2],
        component_types: vec![gl::FLOAT, gl::FLOAT, gl::FLOAT],
        component_offsets: vec![0, 3],
        component_strides: vec![5, 5],
    };
    let mut wall3_mesh = Mesh::new(wall3.vertices, wall3.indices, wall3_attr);

    let wall3_texture_desc =
        TextureDescriptor::new(program_wall3.id, "texture1", "src/brick_wall.jpg", gl::RGB);
    
    let projection_uniform = UniformDescriptor::new(program_wall3.id, "projection");
    let camera_uniform = UniformDescriptor::new(program_wall3.id, "look_at");

    wall3_mesh.add_uniform(projection_uniform);
    wall3_mesh.add_uniform(camera_uniform);

    wall3_mesh.add_texture(wall3_texture_desc);

    let mut wall3_model = Model::new();
    wall3_model.add_mesh(wall3_mesh);
    wall3_model.attach_program(program_wall3);

    app.add_model(wall3_model);


    let wall4 = Quad::new(5.0, 0.6, (-1.25, 0.0, 0.0), (-1.25,0.0,0.0), (2.0,1.0));
    let wall4_attr = AttributesDescriptor {
        component_groups: 2,
        component_nums: vec![3, 2],
        component_types: vec![gl::FLOAT, gl::FLOAT, gl::FLOAT],
        component_offsets: vec![0, 3],
        component_strides: vec![5, 5],
    };

    let mut wall4_mesh = Mesh::new(wall4.vertices, wall4.indices, wall4_attr);

    let wall4_texture_desc =
        TextureDescriptor::new(program_wall4.id, "texture1", "src/brick_wall.jpg", gl::RGB);
    
    let projection_uniform = UniformDescriptor::new(program_wall4.id, "projection");
    let camera_uniform = UniformDescriptor::new(program_wall4.id, "look_at");

    wall4_mesh.add_uniform(projection_uniform);
    wall4_mesh.add_uniform(camera_uniform);

    wall4_mesh.add_texture(wall4_texture_desc);

    let mut wall4_model = Model::new();
    wall4_model.add_mesh(wall4_mesh);
    wall4_model.attach_program(program_wall4);

    app.add_model(wall4_model);

    app.render_models();
}
