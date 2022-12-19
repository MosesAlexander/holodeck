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

    pub fn render_vaos(&mut self) {
        let mut gradient1 = 0.0;
        let mut gradient2 = 0.5;
        let mut gradient3 = 1.0;
        let mut sign1 = 1.0;
        let mut sign2 = 1.0;
        let mut sign3 = 1.0;

        let mut cur_off_x: f32 = 0.0;
        let mut cur_off_y: f32 = 0.0;
        let mut mixvalue: f32 = 0.2;
        let mut moving_up: bool = false;
        let mut moving_down: bool = false;
        let mut moving_left: bool = false;
        let mut moving_right: bool = false;
        let mut x_angle_multiplier: f32 = 0.0;
        let mut y_angle_multiplier: f32 = 0.0;
        let mut z_angle_multiplier: f32 = 0.0;
        let mut x_rot_cwise = false;
        let mut x_rot_ccwise = false;
        let mut y_rot_cwise = false;
        let mut y_rot_ccwise = false;
        let mut z_rot_cwise = false;
        let mut z_rot_ccwise = false;
        let mut reset_all_angles = false;

        let mut mixvalue_grow = false;
        let mut mixvalue_shrink = false;

        /* 
        let orthographic_projection_matrix = Mat4::orthographic_rh_gl(0.0, 800.0, 0.0, 600.0, 0.1, 100.0);
        println!("Orthographic projection matrix:\n{:?}", orthographic_projection_matrix);

        let perspective_projection_matrix = Mat4::perspective_rh_gl(f32::to_radians(45.0), 800.0/600.0, 0.1, 100.0);
        println!("Perspective projection matrix:\n{:?}", perspective_projection_matrix);
        */
        while !self.window.should_close() {
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            for (_, event) in glfw::flush_messages(&self.events) {
                handle_window_event(&mut self.window,
                    event,
                    &mut moving_up, 
                    &mut moving_down, 
                    &mut moving_left, 
                    &mut moving_right, 
                    &mut x_rot_cwise, 
                    &mut x_rot_ccwise,
                    &mut y_rot_cwise, 
                    &mut y_rot_ccwise,
                    &mut z_rot_cwise, 
                    &mut z_rot_ccwise,
                    &mut reset_all_angles,
                    &mut mixvalue_grow,
                    &mut mixvalue_shrink,
                );
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

            if x_rot_ccwise == true {
                x_angle_multiplier += 0.01;
            }
            if x_rot_cwise == true {
                x_angle_multiplier -= 0.01;
            }

            if y_rot_ccwise == true {
                y_angle_multiplier += 0.01;
            }
            if y_rot_cwise == true {
                y_angle_multiplier -= 0.01;
            }

            if z_rot_ccwise == true {
                z_angle_multiplier += 0.01;
            }
            if z_rot_cwise == true {
                z_angle_multiplier -= 0.01;
            }

            if reset_all_angles == true {
                x_angle_multiplier = 0.0;
                y_angle_multiplier = 0.0;
                z_angle_multiplier = 0.0
            }

            let rotate_about_x_matrix = Mat4::from_rotation_x(std::f32::consts::PI * x_angle_multiplier);
            let rotate_about_y_matrix = Mat4::from_rotation_y(std::f32::consts::PI * y_angle_multiplier);
            let rotate_about_z_matrix = Mat4::from_rotation_z(std::f32::consts::PI * z_angle_multiplier);
            let translation_matrix = Mat4::from_translation(Vec3::new(cur_off_x, cur_off_y, 0.0));

            self.vertex_descriptors[1].textures[0].set_active_texture(0);
            self.vertex_descriptors[1].textures[1].set_active_texture(1);

            if mixvalue_grow == true {
                mixvalue+=0.02;
            }
            if mixvalue_shrink == true {
                mixvalue-=0.02;
            }

            self.vertex_descriptors[1].uniforms[0].update(UniformPackedParam::UniformMatrix4FV(Uniform4FVMatrix(rotate_about_x_matrix)));
            self.vertex_descriptors[1].uniforms[1].update(UniformPackedParam::UniformMatrix4FV(Uniform4FVMatrix(rotate_about_y_matrix)));
            self.vertex_descriptors[1].uniforms[2].update(UniformPackedParam::UniformMatrix4FV(Uniform4FVMatrix(rotate_about_z_matrix)));
            self.vertex_descriptors[1].uniforms[3].update(UniformPackedParam::UniformMatrix4FV(Uniform4FVMatrix(translation_matrix)));
            self.vertex_descriptors[1].uniforms[4].update(UniformPackedParam::Uniform1F(Uniform1FParam(mixvalue)));
            
            self.vertex_descriptors[1].render();

            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }

    pub fn add_vertex_descriptor(&mut self, descriptor: VertexDescriptor) {
        self.vertex_descriptors.push(descriptor);
    }

}

fn handle_window_event(window: &mut glfw::Window,
                    event: glfw::WindowEvent,
                    moving_up: &mut bool,
                    moving_down: &mut bool,
                    moving_left: &mut bool,
                    moving_right: &mut bool,
                    x_rotate_cwise: &mut bool,
                    x_rotate_ccwise: &mut bool,
                    y_rotate_cwise: &mut bool,
                    y_rotate_ccwise: &mut bool,
                    z_rotate_cwise: &mut bool,
                    z_rotate_ccwise: &mut bool,
                    reset_all_angles: &mut bool,
                    mixvalue_grow: &mut bool,
                    mixvalue_shrink: &mut bool) {
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


        glfw::WindowEvent::Key(Key::T, _, Action::Press, _ )  => {
            *reset_all_angles = true;
		}
        glfw::WindowEvent::Key(Key::T, _, Action::Release, _ )  => {
            *reset_all_angles = false;
		}

        glfw::WindowEvent::Key(Key::I, _, Action::Press, _ )  => {
            *x_rotate_cwise = true;
		}
        glfw::WindowEvent::Key(Key::I, _, Action::Release, _ )  => {
            *x_rotate_cwise = false;
		}
        glfw::WindowEvent::Key(Key::U, _, Action::Press, _ )  => {
            *x_rotate_ccwise = true;
		}
        glfw::WindowEvent::Key(Key::U, _, Action::Release, _ )  => {
            *x_rotate_ccwise = false;
		}

        glfw::WindowEvent::Key(Key::K, _, Action::Press, _ )  => {
            *y_rotate_cwise = true;
		}
        glfw::WindowEvent::Key(Key::K, _, Action::Release, _ )  => {
            *y_rotate_cwise = false;
		}
        glfw::WindowEvent::Key(Key::J, _, Action::Press, _ )  => {
            *y_rotate_ccwise = true;
		}
        glfw::WindowEvent::Key(Key::J, _, Action::Release, _ )  => {
            *y_rotate_ccwise = false;
		}

        glfw::WindowEvent::Key(Key::M, _, Action::Press, _ )  => {
            *z_rotate_cwise = true;
		}
        glfw::WindowEvent::Key(Key::M, _, Action::Release, _ )  => {
            *z_rotate_cwise = false;
		}
        glfw::WindowEvent::Key(Key::N, _, Action::Press, _ )  => {
            *z_rotate_ccwise = true;
		}
        glfw::WindowEvent::Key(Key::N, _, Action::Release, _ )  => {
            *z_rotate_ccwise = false;
		}

        
        glfw::WindowEvent::Key(Key::E, _, Action::Press, _ ) => {
            *mixvalue_grow = true;
		}
        glfw::WindowEvent::Key(Key::E, _, Action::Release, _ ) => {
            *mixvalue_grow = false;
		}
		glfw::WindowEvent::Key(Key::Q, _, Action::Press, _ ) => {
            *mixvalue_shrink = true;
		}
		glfw::WindowEvent::Key(Key::Q, _, Action::Release, _ ) => {
            *mixvalue_shrink = false;
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
