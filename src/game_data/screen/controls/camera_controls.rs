use miniquad::{KeyCode, KeyMods};

use crate::game_data::screen::{Camera, CameraData, camera_controls, renderer::camera, screen_mananager::ScreenManager};

pub struct CameraControls {
    mouse_cords: [f32; 2],
}

/*
####################
## CameraControls ##
####################




*/

impl CameraControls {
    
    

    //=====================================
    // Init functions
    //=====================================
    pub fn new() -> Self {
        Self {
            mouse_cords: [0.0, 0.0]
        }
    }


    //=====================================
    // Key input
    //=====================================

    pub fn mouse_motion_event(&mut self, screen_manager: &mut ScreenManager, mouse_cords: [f32; 2]) {

    }

    pub fn mouse_button_down_event(&mut self, screen_manager: &mut ScreenManager, x_cor: f32, y_cor: f32) {

    }

    pub fn key_down_event(&mut self, camera: &mut Camera, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        let camera_data = camera.get_mut_camera_data();
        match keycode {
            KeyCode::W | KeyCode::Up => {
                    camera_data.mod_y_cam_cor(0.01);
                }
                KeyCode::A | KeyCode::Right => {
                    camera_data.mod_x_cam_cor(0.01);
                }   
                KeyCode::S | KeyCode::Down => {
                    camera_data.mod_y_cam_cor(-0.01);
                }
                KeyCode::D | KeyCode::Left => {
                    camera_data.mod_x_cam_cor(-0.01);
                }
                _ => {}
        }
    }

    pub fn mouse_wheel_event(&mut self, screen_mananager: &mut ScreenManager, x_scroll_distance: f32, y_scroll_distance: f32) {

    }
}