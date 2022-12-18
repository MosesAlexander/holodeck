extern crate glfw;

use std::ffi::{CString, c_int, c_void};
use glfw::{Action, Context, Key, WindowEvent, Window, Glfw};
use std::sync::mpsc::Receiver;
use glfw::ffi::GLFWwindow;
use stb_image::stb_image::bindgen::*;
use glam::*;
use crate::uniform::*;
use crate::vertex::*;

use crate::Program;
use crate::gl;
use crate::vertex::VertexDescriptor;

extern fn framebuffer_size_callback(_window: *mut GLFWwindow, width: i32, height: i32) {
	unsafe {
		gl::Viewport(0,0,width,height);
	}
}

pub const VERTEX_SHADER: gl::types::GLenum = gl::VERTEX_SHADER;
pub const FRAGMENT_SHADER: gl::types::GLenum = gl::FRAGMENT_SHADER;

pub struct Application {
    program_ids: Vec<gl::types::GLuint>,
    vaos: Vec<gl::types::GLuint>,
    textures: Vec<gl::types::GLuint>,
    glfw: Glfw,
    window: Window,
    events: Receiver<(f64, WindowEvent)>,
    vertex_descriptors: Vec<VertexDescriptor>,
}

impl Application {
    pub fn new() -> Application {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(3,3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        let (mut window, mut events) = glfw.create_window(800, 600, "MyOpenGL", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();

        // the supplied function must be of the type:
        // `&fn(symbol: &'static str) -> *const std::os::raw::c_void`
        // `window` is a glfw::Window
        gl::load_with(|s| window.get_proc_address(s) as *const _);

        // loading a specific function pointer
        gl::Viewport::load_with(|s| window.get_proc_address(s) as *const _);

        unsafe {
            gl::Viewport(0,0,800,600);
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        }

        unsafe {
            glfw::ffi::glfwSetFramebufferSizeCallback(window.window_ptr(), Some(framebuffer_size_callback));
        }

        Application {
                    program_ids: Vec::new(),
                    vaos: Vec::new(),
                    textures: Vec::new(),
                    glfw: glfw,
                    window:window,
                    events: events,
                    vertex_descriptors: Vec::new()}
    }

    pub fn add_program(&mut self, program: &Program) {
        self.program_ids.push(program.id);
    }

    pub fn use_program_at_index(&self, idx: usize) {
        unsafe {
            gl::UseProgram(self.program_ids[idx]);
        }
    }

    pub fn generate_indexed_triangles(&mut self, vertices: &Vec<f32>,
                                   indexes_two_triangles: &Vec<u32>,
                                   vertices_third_triangle: &Vec<f32>,
                                   indexes_third_triangle: &Vec<u32>) {
        let mut vao: gl::types::GLuint = 0;
        let mut vbo: gl::types::GLuint = 0;
        let mut ebo: gl::types::GLuint = 0;
        let mut vao2: gl::types::GLuint = 0;
        let mut vbo2: gl::types::GLuint = 0;
        let mut ebo2: gl::types::GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );

            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indexes_two_triangles.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indexes_two_triangles.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            
            gl::VertexAttribPointer(0, 3, 
                gl::FLOAT,
                gl::FALSE,
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null()
            );
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(1, 3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid);
            gl::EnableVertexAttribArray(1);

            gl::GenVertexArrays(1, &mut vao2);
            gl::BindVertexArray(vao2);
            
            gl::GenBuffers(1, &mut vbo2);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo2);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices_third_triangle.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices_third_triangle.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );

            gl::GenBuffers(1, &mut ebo2);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo2);

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indexes_third_triangle.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indexes_third_triangle.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            
            gl::VertexAttribPointer(0, 3,
                gl::FLOAT, 
                gl::FALSE, 
                (8 * std::mem::size_of::<f32>()) as gl::types::GLint, 
                std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(1, 3,
                    gl::FLOAT,
                    gl::FALSE,
                    (8 * std::mem::size_of::<f32>()) as gl::types::GLint,
                    (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid);
            gl::EnableVertexAttribArray(1);

            gl::VertexAttribPointer(2, 2,
                                    gl::FLOAT,
                                gl::FALSE,
                                (8 * std::mem::size_of::<f32>()) as gl::types::GLint,
                                (6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid);
            gl::EnableVertexAttribArray(2);
        }

        // Texture generation part
        let mut width: c_int = 0;
        let mut height: c_int = 0;
        let mut nr_channels: c_int = 0;
        let mut texture1_id: gl::types::GLuint = 0;
        let mut path = CString::new("src/stallman.jpg").unwrap();

        unsafe {

            gl::GenTextures(1, &mut texture1_id);

            gl::BindTexture(gl::TEXTURE_2D, texture1_id);

            // Set the texture wrapping/filtering options on the currently bound texture object
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            stbi_set_flip_vertically_on_load(1);
            let buffer = stbi_load(path.as_ptr(), &mut width, &mut height, &mut nr_channels, 0);

            if (!buffer.is_null()) {
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, width, height, 0,
                                gl::RGB, gl::UNSIGNED_BYTE, buffer as *const c_void);
                gl::GenerateMipmap(gl::TEXTURE_2D);
                stbi_image_free(buffer as *mut c_void);
            } else {
                println!("Failed to load texture!");

            }

        }

        let mut texture2_id: gl::types::GLuint = 0;
        path = CString::new("src/gnu.png").unwrap();

        unsafe {

            gl::GenTextures(1, &mut texture2_id);

            gl::BindTexture(gl::TEXTURE_2D, texture2_id);

            // Set the texture wrapping/filtering options on the currently bound texture object
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            stbi_set_flip_vertically_on_load(1);
            let buffer = stbi_load(path.as_ptr(), &mut width, &mut height, &mut nr_channels, 0);

            if (!buffer.is_null()) {
                // use RGBA format, png supports transparency
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, width, height, 0,
                                gl::RGBA, gl::UNSIGNED_BYTE, buffer as *const c_void);
                gl::GenerateMipmap(gl::TEXTURE_2D);
                stbi_image_free(buffer as *mut c_void);
            } else {
                println!("Failed to load texture!");

            }

        }

        self.vaos.push(vao);
        self.vaos.push(vao2);
        self.textures.push(texture1_id);
        self.textures.push(texture2_id);
    }

    pub fn render_vaos(&mut self) {
        let mut gradient1 = 0.0;
        let mut gradient2 = 0.5;
        let mut gradient3 = 1.0;
        let mut sign1 = 1.0;
        let mut sign2 = 1.0;
        let mut sign3 = 1.0;

        let position_offset: gl::types::GLint;
        let mut cur_off_x: f32 = 0.0;
        let mut cur_off_y: f32 = 0.0;
        let mut texture1_id: gl::types::GLint = 0;
        let mut texture2_id: gl::types::GLint = 0;
        let mut mixvalue_id: gl::types::GLint = 0;
        let mut mixvalue: f32 = 0.2;
        let mut transform_id: gl::types::GLint = 0;
        let mut moving_up: bool = false;
        let mut moving_down: bool = false;
        let mut moving_left: bool = false;
        let mut moving_right: bool = false;
        let mut translate_id: gl::types::GLint = 0;
        let mut angle_multiplier: f32 = 0.0;
        let mut rot_cwise = false;
        let mut rot_ccwise = false;

        let mut sign = 1.0;

        while !self.window.should_close() {
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            for (_, event) in glfw::flush_messages(&self.events) {
                handle_window_event(&mut self.window, event, &mut moving_up, &mut moving_down, &mut moving_left, &mut moving_right, &mut mixvalue, &mut rot_cwise, &mut rot_ccwise);
            }

            if gradient1 <= 0.0 || gradient1 >= 1.0 {
                sign1*=-1.0;
            }
            if gradient2 <= 0.0 || gradient2 >= 1.0 {
                sign2*=-1.0;
            }
            if gradient3 <= 0.0 || gradient3 >= 1.0 {
                sign3*=-1.0;
            }
            //R: g3g2g1
            //G: g1g3g2
            //B: g1g2g3
            gradient1 = gradient1 + (0.005*sign1);
            gradient2 = gradient2 + (0.005*sign2);
            gradient3 = gradient3 + (0.005*sign3);

            self.use_program_at_index(0);
            self.vertex_descriptors[0].bind();
            
            self.vertex_descriptors[0].uniforms[0].update(UniformPackedParam::Uniform3F(Uniform3FParam(gradient1,gradient3,gradient2)));
            self.vertex_descriptors[0].uniforms[1].update(UniformPackedParam::Uniform3F(Uniform3FParam(gradient3,gradient2,gradient1)));
            self.vertex_descriptors[0].uniforms[2].update(UniformPackedParam::Uniform3F(Uniform3FParam(gradient1,gradient2,gradient3)));
            self.vertex_descriptors[0].uniforms[3].update(UniformPackedParam::Uniform3F(Uniform3FParam(gradient1,gradient3,gradient2)));

            self.vertex_descriptors[0].render();
           // unsafe {
             //   gl::BindVertexArray(0);
           // }

            self.use_program_at_index(1);

            self.vertex_descriptors[1].bind();

            if moving_down == true {
                cur_off_y-=0.02;
            }
            if moving_up == true {
                cur_off_y+=0.02;
            }
            if moving_left == true {
                cur_off_x-=0.02;
            }
            if moving_right == true {
                cur_off_x+=0.02;
            }
            if rot_ccwise == true {
                angle_multiplier += 0.01;
            }
            if rot_cwise == true {
                angle_multiplier -= 0.01;
            }

            let transform_matrix = Mat4::from_rotation_z(std::f32::consts::PI * angle_multiplier);
            let translation_matrix = Mat4::from_translation(Vec3::new(cur_off_x, cur_off_y, 0.0));

            self.vertex_descriptors[1].textures[0].set_active_texture(0);
            self.vertex_descriptors[1].textures[1].set_active_texture(1);

            self.vertex_descriptors[1].uniforms[0].update(UniformPackedParam::UniformMatrix4FV(Uniform4FVMatrix(transform_matrix)));
            self.vertex_descriptors[1].uniforms[1].update(UniformPackedParam::UniformMatrix4FV(Uniform4FVMatrix(translation_matrix)));
            self.vertex_descriptors[1].uniforms[2].update(UniformPackedParam::Uniform1F(Uniform1FParam(mixvalue)));
            
            self.vertex_descriptors[1].render();

            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }

    pub fn add_vertex_descriptor(&mut self, descriptor: VertexDescriptor) {
        self.vertex_descriptors.push(descriptor);
    }

}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent, moving_up: &mut bool, moving_down: &mut bool, moving_left: &mut bool, moving_right: &mut bool, mixvalue: &mut f32, rotate_clockwise: &mut bool, rotate_counterclockwise: &mut bool) {
	match event {
		glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
			window.set_should_close(true)
		}

		glfw::WindowEvent::Key(Key::Up, _, Action::Press, _ )  => {
            *moving_up = true;
		}
		glfw::WindowEvent::Key(Key::Left, _, Action::Press, _ ) => {
            *moving_left = true;
		}
		glfw::WindowEvent::Key(Key::Right, _, Action::Press, _ ) => {
            *moving_right = true;
		}
		glfw::WindowEvent::Key(Key::Down, _, Action::Press, _ ) => {
            *moving_down = true;
		}
		glfw::WindowEvent::Key(Key::Up, _, Action::Release, _ )  => {
            *moving_up = false;
		}
		glfw::WindowEvent::Key(Key::Left, _, Action::Release, _ ) => {
            *moving_left = false;
		}
		glfw::WindowEvent::Key(Key::Right, _, Action::Release, _ ) => {
            *moving_right = false;
		}
		glfw::WindowEvent::Key(Key::Down, _, Action::Release, _ ) => {
            *moving_down = false;
		}
        glfw::WindowEvent::Key(Key::K, _, Action::Press, _ )  => {
            *rotate_clockwise = true;
		}
        glfw::WindowEvent::Key(Key::K, _, Action::Release, _ )  => {
            *rotate_clockwise = false;
		}
        glfw::WindowEvent::Key(Key::J, _, Action::Press, _ )  => {
            *rotate_counterclockwise = true;
		}
        glfw::WindowEvent::Key(Key::J, _, Action::Release, _ )  => {
            *rotate_counterclockwise = false;
		}

        
        glfw::WindowEvent::Key(Key::I, _, Action::Repeat, _ ) | glfw::WindowEvent::Key(Key::I, _, Action::Press, _ ) => {
            *mixvalue+=0.01;
		}
		glfw::WindowEvent::Key(Key::U, _, Action::Repeat, _ ) | glfw::WindowEvent::Key(Key::U, _, Action::Press, _ ) => {
            *mixvalue-=0.01;
		}
		glfw::WindowEvent::Key(Key::W, _, Action::Press, _ ) => {
			unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
			}
		}
		glfw::WindowEvent::Key(Key::F, _, Action::Press, _ ) => {
			unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL)
			}
		}
		_ => {}
	}
}
