use std::os::raw::c_void;
use std::path::Path;
use std::sync::mpsc::Receiver;

use gl;
extern crate glfw;

use self::glfw::{Key, Action};

use image;
use image::GenericImage;
use image::DynamicImage::*;

use crate::camera::{*, CameraMovement::*};

pub fn process_events(events: &Receiver<(f64, glfw::WindowEvent)>,
                      firstMouse: &mut bool,
                      lastX: &mut f32,
                      lastY: &mut f32,
                      camera: &mut Camera) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::CursorPos(xpos, ypos) => {
                let (xpos, ypos) = (xpos as f32, ypos as f32);
                if *firstMouse {
                    *lastX = xpos;
                    *lastY = ypos;
                    *firstMouse = false;
                }

                let xoffset = xpos - *lastX;
                let yoffset = *lastY - ypos; // reversed since y-coordinates go from bottom to top

                *lastX = xpos;
                *lastY = ypos;

                camera.process_mouse_movement(xoffset, yoffset, true);
            }
            glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                camera.process_mouse_scroll(yoffset as f32);
            }
            _ => {}
        }
    }
}

/// Input processing function as introduced in 1.7.4 (Camera Class) and used in
/// most later tutorials
pub fn process_input(window: &mut glfw::Window, delta_time: f32, camera: &mut Camera) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true)
    }

    if window.get_key(Key::W) == Action::Press {
        camera.process_keyboard(FORWARD, delta_time);
    }
    if window.get_key(Key::S) == Action::Press {
        camera.process_keyboard(BACKWARD, delta_time);
    }
    if window.get_key(Key::A) == Action::Press {
        camera.process_keyboard(LEFT, delta_time);
    }
    if window.get_key(Key::D) == Action::Press {
        camera.process_keyboard(RIGHT, delta_time);
    }
}