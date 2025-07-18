mod camera;
mod shader;
mod input;
mod macros;
mod model;
mod mesh;

extern crate glfw;
extern crate gl;
extern crate imgui_opengl_renderer;
extern crate imgui;

use std::ffi::CStr;
use glfw::{Action, Context, Key};
use std::ffi::CString;
use std::ptr;
use imgui::*;
use cgmath::*;
use crate::camera::Camera;
use crate::input::{process_input, process_events};
use crate::model::Model;
use crate::shader::Shader;


const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    
    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "OpenGL + GLFW in Rust", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);

    // tell GLFW to capture our mouse
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Imgui
    // let mut imgui = imgui::Context::create();
    // 
    // let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s|  window.get_proc_address(s) as *const _);
    // 

    unsafe { gl::Enable(gl::DEPTH_TEST); }
    
    // let vertices: [f32; 9] = [
    //     -0.5, -0.5, 0.0,
    //     0.5, -0.5, 0.0,
    //     0.0,  0.5, 0.0,
    // ];
    // 
    // let mut vbo: u32 = 0;
    // let mut vao: u32 = 0;
    // unsafe {
    //     gl::GenVertexArrays(1, &mut vao);
    //     gl::GenBuffers(1, &mut vbo);
    // 
    //     gl::BindVertexArray(vao);
    //     gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    //     gl::BufferData(gl::ARRAY_BUFFER,
    //                    (vertices.len() * std::mem::size_of::<f32>()) as isize,
    //                    vertices.as_ptr() as *const _,
    //                    gl::STATIC_DRAW);
    // 
    //     gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, ptr::null());
    //     gl::EnableVertexAttribArray(0);
    // }
    
    let shader_program  = Shader::new("./assets/shaders/model.vert", "./assets/shaders/model.frag");

    let entity = Model::new("./assets/models/nanosuit/nanosuit.obj");
    
    let mut camera = Camera {
        position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };
    
    let mut firstMouse = true;
    let mut lastX: f32 = SCR_WIDTH as f32 / 2.0;
    let mut lastY: f32 = SCR_HEIGHT as f32 / 2.0;

    // timing
    let mut deltaTime: f32; // time between current frame and last frame
    let mut lastFrame: f32 = 0.0;


    while !window.should_close() {
        let currentFrame = glfw.get_time() as f32;
        deltaTime = currentFrame - lastFrame;
        lastFrame = currentFrame;

        // events
        // -----
        process_events(&events, &mut firstMouse, &mut lastX, &mut lastY, &mut camera);

        // input
        // -----
        process_input(&mut window, deltaTime, &mut camera);

        // Start a new ImGui frame
        // let (width, height) = window.get_framebuffer_size();
        // imgui.io_mut().display_size = [width as f32, height as f32];
        // let ui = imgui.frame();

        // Show ImGui demo window
        //     ui.show_demo_window(&mut true);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    
            gl::UseProgram(shader_program.id);

            let projection: Matrix4<f32> = perspective(Deg(camera.zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            let view = camera.get_view_matrix();
            shader_program.set_mat4(c_str!("projection"), &projection);
            shader_program.set_mat4(c_str!("view"), &view);

            // render the loaded model
            let mut model = Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0)); // translate it down so it's at the center of the scene
            model = model * Matrix4::from_scale(0.2);
            shader_program.set_mat4(c_str!("model"), &model);
            entity.draw(&shader_program);
        }
        // renderer.render(&mut imgui);
        window.swap_buffers();
        glfw.poll_events();
    }
    
}