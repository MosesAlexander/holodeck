extern crate glfw;

use crate::text::TextManager;
use crate::uniform::*;

use glam::*;
use glfw::ffi::{GLFWwindow, glfwSetInputMode, CURSOR, CURSOR_DISABLED, glfwGetCursorPos};
use glfw::{Action, Context, Glfw, Key, Window, WindowEvent};


use std::sync::mpsc::Receiver;

use crate::gl::{self};
use crate::vertex::{VertexDescriptor, Model};
use crate::Program;

extern crate freetype;

extern "C" fn framebuffer_size_callback(_window: *mut GLFWwindow, width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}

pub const VERTEX_SHADER: gl::types::GLenum = gl::VERTEX_SHADER;
pub const FRAGMENT_SHADER: gl::types::GLenum = gl::FRAGMENT_SHADER;

pub struct Application {
    program_ids: Vec<gl::types::GLuint>,
    vaos: Vec<gl::types::GLuint>,
    textures: Vec<gl::types::GLuint>,
    models: Vec<Model>,
    glfw: Glfw,
    window: Window,
    events: Receiver<(f64, WindowEvent)>,
    vertex_descriptors: Vec<VertexDescriptor>,
    text_manager: Option<TextManager>,
}

impl Application {
    pub fn new() -> Application {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let (mut window, events) = glfw
            .create_window(1024, 768, "rust-opengl", glfw::WindowMode::Windowed)
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
            gl::Viewport(0, 0, 1024, 768);
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        }

        unsafe {
            glfw::ffi::glfwSetFramebufferSizeCallback(
                window.window_ptr(),
                Some(framebuffer_size_callback),
            );
        }

        Application {
            program_ids: Vec::new(),
            models: Vec::new(),
            vaos: Vec::new(),
            textures: Vec::new(),
            glfw: glfw,
            window: window,
            events: events,
            vertex_descriptors: Vec::new(),
            text_manager: None,
        }
    }

    pub fn add_program(&mut self, program: &Program) {
        self.program_ids.push(program.id);
    }

    pub fn attach_text_manager(&mut self, text_manager: TextManager) {
        self.text_manager = Some(text_manager);
    }

    pub fn use_program_at_index(&self, idx: usize) {
        unsafe {
            gl::UseProgram(self.program_ids[idx]);
        }
    }


    pub fn render_models(&mut self) {
        let mut cur_off_x: f32 = 0.0;
        let mut cur_off_y: f32 = 0.0;
        let mut cur_off_z: f32 = -0.4;
        let camera_cur_off_x: f32 = 0.0;
        let camera_cur_off_y: f32 = 0.2;
        let camera_cur_off_z: f32 = 2.0;
        let mut mixvalue: f32 = 0.5;
        let mut moving_up: bool = false;
        let mut moving_down: bool = false;
        let mut moving_left: bool = false;
        let mut moving_right: bool = false;
        let mut moving_in: bool = false;
        let mut moving_out: bool = false;
        let mut camera_moving_up: bool = false;
        let mut camera_moving_down: bool = false;
        let mut camera_moving_left: bool = false;
        let mut camera_moving_right: bool = false;
        let mut camera_moving_forwards: bool = false;
        let mut camera_moving_backwards: bool = false;
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
        let mut yaw: f32 = -90.0;
        let mut pitch: f32 = 0.0;
        let mut zoom_in = false;
        let mut zoom_out = false;
        let mut reset_zoom = false;
        let mut fov_val = 45.0;

        let mut mixvalue_grow = false;
        let mut mixvalue_shrink = false;

        let mut last_cursor_x: f64 = 400.0;
        let mut last_cursor_y: f64 = 300.0;
        let mut current_cursor_x: f64 = 0.0;
        let mut current_cursor_y: f64 = 0.0;

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        unsafe {
            glfwSetInputMode(self.window.window_ptr(), CURSOR, CURSOR_DISABLED);
        }

        self.models[0].use_program();

        let mut perspective_projection_matrix =
            Mat4::perspective_rh_gl(f32::to_radians(fov_val), 1024.0 / 768.0, 0.1, 100.0);

        self.models[0].meshes[0].uniforms[5].update(UniformPackedParam::UniformMatrix4FV(
            Uniform4FVMatrix(perspective_projection_matrix),
        ));

        // Initial position
        let mut camera_position = Vec3::new(camera_cur_off_x, camera_cur_off_y, camera_cur_off_z);

        while !self.window.should_close() {
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }
            for (_, event) in glfw::flush_messages(&self.events) {
                handle_window_event(
                    &mut self.window,
                    event,
                    &mut moving_up,
                    &mut moving_down,
                    &mut moving_left,
                    &mut moving_right,
                    &mut moving_in,
                    &mut moving_out,
                    &mut x_rot_cwise,
                    &mut x_rot_ccwise,
                    &mut y_rot_cwise,
                    &mut y_rot_ccwise,
                    &mut z_rot_cwise,
                    &mut z_rot_ccwise,
                    &mut reset_all_angles,
                    &mut mixvalue_grow,
                    &mut mixvalue_shrink,
                    &mut camera_moving_forwards,
                    &mut camera_moving_backwards,
                    &mut camera_moving_down,
                    &mut camera_moving_up,
                    &mut camera_moving_left,
                    &mut camera_moving_right,
                    &mut zoom_in,
                    &mut zoom_out,
                    &mut reset_zoom,
                );
            }

            if moving_in == true {
                cur_off_z += 0.02;
            }
            if moving_out == true {
                cur_off_z -= 0.02;
            }
            if moving_down == true {
                cur_off_y -= 0.02;
            }
            if moving_up == true {
                cur_off_y += 0.02;
            }
            if moving_left == true {
                cur_off_x -= 0.02;
            }
            if moving_right == true {
                cur_off_x += 0.02;
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

            if zoom_in == true {
                if fov_val > 0.0 {
                    fov_val -= 0.2;
                }
            } 

            if zoom_out == true {
                if fov_val < 360.0 {
                    fov_val += 0.2;
                }
            }

            if reset_zoom == true {
                fov_val = 45.0;
            }

           
            if zoom_out == true || zoom_in == true || reset_zoom == true {
                perspective_projection_matrix =
                    Mat4::perspective_rh_gl(f32::to_radians(fov_val), 1024.0 / 768.0, 0.1, 100.0);

                #[cfg(feature = "printdebugs")]
                println!("zoom_in/out_perspective: {:?}", perspective_projection_matrix);

                self.models[0].use_program();
                for mesh in self.models[0].meshes.iter_mut() {
                    mesh.uniforms[5].update(UniformPackedParam::UniformMatrix4FV(
                        Uniform4FVMatrix(perspective_projection_matrix),
                    ));
                }
            }

            let rotate_about_x_matrix =
                Mat4::from_rotation_x(std::f32::consts::PI * x_angle_multiplier);
            let rotate_about_y_matrix =
                Mat4::from_rotation_y(std::f32::consts::PI * y_angle_multiplier);
            let rotate_about_z_matrix =
                Mat4::from_rotation_z(std::f32::consts::PI * z_angle_multiplier);
            let translation_matrix =
                Mat4::from_translation(Vec3::new(cur_off_x, cur_off_y, cur_off_z));

            self.models[0].meshes[0].textures[0].set_active_texture(0);
            self.models[0].meshes[0].textures[1].set_active_texture(1);

            if mixvalue_grow == true {
                mixvalue += 0.02;
            }
            if mixvalue_shrink == true {
                mixvalue -= 0.02;
            }

            unsafe {
                glfwGetCursorPos(self.window.window_ptr(), &mut current_cursor_x as *mut f64, &mut current_cursor_y as *mut f64);
            }
            let cursor_x_diff = last_cursor_x - current_cursor_x;
            last_cursor_x = current_cursor_x;
            let cursor_y_diff = last_cursor_y - current_cursor_y;
            last_cursor_y = current_cursor_y;

            yaw -= 0.03 * cursor_x_diff as f32;
            pitch += 0.03 * cursor_y_diff as f32;
            if pitch < -89.95 {
                pitch = -89.95;
            }

            if pitch > 89.95 {
                pitch = 89.95;
            }

            #[cfg(feature = "printdebugs")]
            println!("cur_x: {} cur_y: {} last_x: {} last_y: {} diff_x: {} diff_y: {}",
                    current_cursor_x, current_cursor_y, last_cursor_x, last_cursor_y, cursor_x_diff,
                    cursor_y_diff);
            #[cfg(feature = "printdebugs")]
            println!("yaw: {} pitch: {}", yaw, pitch);

            // Gram-Schmidt process
            // Positive Z axis leads outside the screen

            let mut direction = Vec3::new(0.0,0.0,0.0);
            direction.x = yaw.to_radians().cos() * pitch.to_radians().cos();
            direction.y = pitch.to_radians().sin();
            direction.z = yaw.to_radians().sin() * pitch.to_radians().cos();

            let camera_front = direction.normalize();

            if camera_moving_forwards == true {
                camera_position += camera_front*0.02;
            }

            if camera_moving_backwards == true {
                camera_position -= camera_front*0.02;
            }

            camera_position.y = 0.2;

            let camera_target = camera_position + camera_front;
            // For the view matrix's coordinate system we want its z-axis
            // to be positive and because by convention (in OpenLG)
            // the camera points towards the neg z-axis we want to negate
            // the direciton vector.
            // the name "direction vector" is a misnomer, since it is actually
            // pointing in the reverse direction of what it is targeting
            let camera_direction = (camera_position - camera_target).normalize();
            //let camera_direction = direction;
            // To get the right-axis do a cross product between up and target
            let c_up = Vec3::new(0.0, 1.0, 0.0);
            let camera_right = c_up.cross(camera_direction).normalize();
            // get up axis by crossing camera direction with camera right
            let camera_up = camera_direction.cross(camera_right);

            if camera_moving_left == true {
                camera_position -= camera_right * 0.009;
            }

            if camera_moving_right == true {
                camera_position += camera_right * 0.009;
            }

            if camera_moving_down == true {
                camera_position.y -= 0.02;
                if camera_position.y < 0.2 {
                    camera_position.y = 0.2;
                }
            }

            if camera_moving_up == true {
                camera_position.y += 0.02;
            }


            #[cfg(feature = "printdebugs")]
            println!("camera_position: {:?}", camera_position);


            // From these 3 vectors we can create a LookAt matrix
            let mut mat_A = Mat4::from_cols(
                                        Vec4::from((camera_right, 0.0)),
                                        Vec4::from((camera_up, 0.0)),
                                        Vec4::from((camera_direction,0.0)),
                                        Vec4::W
            );
            mat_A = mat_A.transpose();

            let mat_B = Mat4::from_cols(
                                    Vec4::X,
                                    Vec4::Y,
                                    Vec4::Z,
                                    Vec4::from((-camera_position, 1.0))

            );

            let LookAt = mat_A * mat_B;

            self.models[0].use_program();
            for mesh in self.models[0].meshes.iter_mut() {
                mesh.uniforms[6].update(UniformPackedParam::UniformMatrix4FV(
                    Uniform4FVMatrix(LookAt)
                ));

                mesh.uniforms[0].update(UniformPackedParam::UniformMatrix4FV(
                    Uniform4FVMatrix(rotate_about_x_matrix),
                ));
                mesh.uniforms[1].update(UniformPackedParam::UniformMatrix4FV(
                    Uniform4FVMatrix(rotate_about_y_matrix),
                ));
                mesh.uniforms[2].update(UniformPackedParam::UniformMatrix4FV(
                    Uniform4FVMatrix(rotate_about_z_matrix),
                ));
                mesh.uniforms[3].update(UniformPackedParam::UniformMatrix4FV(
                    Uniform4FVMatrix(translation_matrix),
                ));
                mesh.uniforms[4].update(UniformPackedParam::Uniform1F(
                    Uniform1FParam(mixvalue)));
                
                mesh.bind_vao();
            }
            self.models[0].render();

            for model in self.models[1..].iter_mut() {
                model.use_program();
                for mesh in model.meshes.iter_mut() {
                    mesh.bind_vao();
                    mesh.textures[0].set_active_texture(0);
                    mesh.uniforms[0].update(UniformPackedParam::UniformMatrix4FV(
                        Uniform4FVMatrix(perspective_projection_matrix),
                    ));

                    mesh.uniforms[1].update(UniformPackedParam::UniformMatrix4FV(
                        Uniform4FVMatrix(LookAt)
                    ));

                }
                model.render();
            }

            self.text_manager.as_ref().unwrap().use_text_program();
            self.text_manager.as_mut().unwrap().render_text("Greetings mortals".to_string(), 25.0, 25.0, 1.0, Vec3::new(0.5, 0.8, 0.2));

            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }

    pub fn add_vertex_descriptor(&mut self, descriptor: VertexDescriptor) {
        self.vertex_descriptors.push(descriptor);
    }

    pub fn add_model(&mut self, model: Model) {
        self.models.push(model);
    }
}

fn handle_window_event(
    window: &mut glfw::Window,
    event: glfw::WindowEvent,
    moving_up: &mut bool,
    moving_down: &mut bool,
    moving_left: &mut bool,
    moving_right: &mut bool,
    moving_in: &mut bool,
    moving_out: &mut bool,
    x_rotate_cwise: &mut bool,
    x_rotate_ccwise: &mut bool,
    y_rotate_cwise: &mut bool,
    y_rotate_ccwise: &mut bool,
    z_rotate_cwise: &mut bool,
    z_rotate_ccwise: &mut bool,
    reset_all_angles: &mut bool,
    mixvalue_grow: &mut bool,
    mixvalue_shrink: &mut bool,
    camera_moving_forwards: &mut bool,
    camera_moving_backwards: &mut bool,
    camera_moving_down: &mut bool,
    camera_moving_up: &mut bool,
    camera_moving_left: &mut bool,
    camera_moving_right: &mut bool,
    zoom_in: &mut bool,
    zoom_out: &mut bool,
    reset_zoom: &mut bool,
) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::Key(Key::C, _, Action::Press, _) => {
            *moving_in = true;
        }
        glfw::WindowEvent::Key(Key::C, _, Action::Release, _) => {
            *moving_in = false;
        }
        glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
            *camera_moving_forwards = true;
        }
        glfw::WindowEvent::Key(Key::W, _, Action::Release, _) => {
            *camera_moving_forwards = false;
        }
        glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
            *camera_moving_backwards = true;
        }
        glfw::WindowEvent::Key(Key::S, _, Action::Release, _) => {
            *camera_moving_backwards = false;
        }
        glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
            *camera_moving_left = true;
        }
        glfw::WindowEvent::Key(Key::A, _, Action::Release, _) => {
            *camera_moving_left = false;
        }
        glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
            *camera_moving_right = true;
        }
        glfw::WindowEvent::Key(Key::D, _, Action::Release, _) => {
            *camera_moving_right = false;
        }
        glfw::WindowEvent::Key(Key::Q, _, Action::Press, _) => {
            *camera_moving_down = true;
        }
        glfw::WindowEvent::Key(Key::Q, _, Action::Release, _) => {
            *camera_moving_down = false;
        }
        glfw::WindowEvent::Key(Key::E, _, Action::Press, _) => {
            *camera_moving_up = true;
        }
        glfw::WindowEvent::Key(Key::E, _, Action::Release, _) => {
            *camera_moving_up = false;
        }
        glfw::WindowEvent::Key(Key::Z, _, Action::Press, _) => {
            *moving_out = true;
        }
        glfw::WindowEvent::Key(Key::Z, _, Action::Release, _) => {
            *moving_out = false;
        }
        glfw::WindowEvent::Key(Key::Up, _, Action::Press, _) => {
            *moving_up = true;
        }
        glfw::WindowEvent::Key(Key::Left, _, Action::Press, _) => {
            *moving_left = true;
        }
        glfw::WindowEvent::Key(Key::Right, _, Action::Press, _) => {
            *moving_right = true;
        }
        glfw::WindowEvent::Key(Key::Down, _, Action::Press, _) => {
            *moving_down = true;
        }
        glfw::WindowEvent::Key(Key::Up, _, Action::Release, _) => {
            *moving_up = false;
        }
        glfw::WindowEvent::Key(Key::Left, _, Action::Release, _) => {
            *moving_left = false;
        }
        glfw::WindowEvent::Key(Key::Right, _, Action::Release, _) => {
            *moving_right = false;
        }
        glfw::WindowEvent::Key(Key::Down, _, Action::Release, _) => {
            *moving_down = false;
        }
        
        glfw::WindowEvent::Key(Key::KpAdd, _, Action::Press, _) => {
            *zoom_in = true;
        }
        glfw::WindowEvent::Key(Key::KpAdd, _, Action::Release, _) => {
            *zoom_in = false;
        }
        glfw::WindowEvent::Key(Key::KpSubtract, _, Action::Press, _) => {
            *zoom_out = true;
        }
        glfw::WindowEvent::Key(Key::KpSubtract, _, Action::Release, _) => {
            *zoom_out = false;
        }
        glfw::WindowEvent::Key(Key::KpMultiply, _, Action::Press, _) => {
            *reset_zoom = true;
        }
        glfw::WindowEvent::Key(Key::KpMultiply, _, Action::Release, _) => {
            *reset_zoom = false;
        }

        glfw::WindowEvent::Key(Key::T, _, Action::Press, _) => {
            *reset_all_angles = true;
        }
        glfw::WindowEvent::Key(Key::T, _, Action::Release, _) => {
            *reset_all_angles = false;
        }

        glfw::WindowEvent::Key(Key::I, _, Action::Press, _) => {
            *x_rotate_cwise = true;
        }
        glfw::WindowEvent::Key(Key::I, _, Action::Release, _) => {
            *x_rotate_cwise = false;
        }
        glfw::WindowEvent::Key(Key::U, _, Action::Press, _) => {
            *x_rotate_ccwise = true;
        }

        glfw::WindowEvent::Key(Key::U, _, Action::Release, _) => {
            *x_rotate_ccwise = false;
        }

        glfw::WindowEvent::Key(Key::K, _, Action::Press, _) => {
            *y_rotate_cwise = true;
        }
        glfw::WindowEvent::Key(Key::K, _, Action::Release, _) => {
            *y_rotate_cwise = false;
        }
        glfw::WindowEvent::Key(Key::J, _, Action::Press, _) => {
            *y_rotate_ccwise = true;
        }
        glfw::WindowEvent::Key(Key::J, _, Action::Release, _) => {
            *y_rotate_ccwise = false;
        }

        glfw::WindowEvent::Key(Key::M, _, Action::Press, _) => {
            *z_rotate_cwise = true;
        }
        glfw::WindowEvent::Key(Key::M, _, Action::Release, _) => {
            *z_rotate_cwise = false;
        }
        glfw::WindowEvent::Key(Key::N, _, Action::Press, _) => {
            *z_rotate_ccwise = true;
        }
        glfw::WindowEvent::Key(Key::N, _, Action::Release, _) => {
            *z_rotate_ccwise = false;
        }

        glfw::WindowEvent::Key(Key::Num3, _, Action::Press, _) => {
            *mixvalue_grow = true;
        }
        glfw::WindowEvent::Key(Key::Num3, _, Action::Release, _) => {
            *mixvalue_grow = false;
        }
        glfw::WindowEvent::Key(Key::Num1, _, Action::Press, _) => {
            *mixvalue_shrink = true;
        }
        glfw::WindowEvent::Key(Key::Num1, _, Action::Release, _) => {
            *mixvalue_shrink = false;
        }

        glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        },
        glfw::WindowEvent::Key(Key::F, _, Action::Press, _) => unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL)
        },
        _ => {}
    }
}
