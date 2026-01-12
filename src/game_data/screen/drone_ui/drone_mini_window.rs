use miniquad::MouseButton;

use crate::game_data::{TextureManager, screen::{ScreenData, camera_data::{self, CameraData}}, tik_manager::drones::drone::Drone, types::DroneUITexture};

pub struct MiniWindow {
    // Drone Data
    ndc_cords: [f32; 2],
    scale: f32,
    pressed: bool,
}

impl MiniWindow {
    pub fn new() -> MiniWindow {
        MiniWindow {
            ndc_cords: [0.0, 0.0],
            scale: 0.0,
            pressed: false,
        }  
    }

    //=====================================
    // Setters / Getters
    //=====================================

    pub fn is_pressed(&self) -> bool {
        return self.pressed;
    }
    pub fn set_pressed(&mut self, pressed: bool) {
        self.pressed = pressed;
    }


    pub fn get_scale(&self) -> f32 {
        return self.scale;
    }
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn get_ndc_cords(&self) -> [f32; 2] {
        return self.ndc_cords;
    }
    
    pub fn update_drone_ndc_cords(&mut self, drone_world_cords: [i32; 3], camera_data: &CameraData) {
        let drone_ndc_cords = camera_data.world_to_ndc_cords(drone_world_cords);
        let centered_ndc_draw_cords = [drone_ndc_cords[0] - self.scale /2.0, drone_ndc_cords[1] - self.scale / 3.0];
        self.ndc_cords = centered_ndc_draw_cords;
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn render(&mut self, texture_manager: &mut TextureManager, camera_data: &CameraData, drone: &Drone) {
        self.update_drone_ndc_cords(drone.get_cords(), camera_data);
        texture_manager.render_drone_ui_element(DroneUITexture::DroneMiniWindow, self.ndc_cords, self.scale);
    }

    //=====================================
    // Controls
    //=====================================

    pub fn handle_motion_event(&mut self, screen_data: &ScreenData) {
        // if on mini_window

    }

    pub fn mouse_button_down_event(&mut self, button: MouseButton, screen_data: &ScreenData) {
        // Gotta calculate if mouse is on window based off scale using src rect y to x racio

        if button == MouseButton::Left {
            let y_scale = DroneUITexture::DroneMiniWindow.get_y_to_x_ratio() * self.scale;
            let end_ndc_cords = [self.ndc_cords[0] + self.scale, self.ndc_cords[1] + y_scale];

            let mouse_ndc = screen_data.get_mouse_ndc_cords();

            let x_in_window = mouse_ndc[0] > self.ndc_cords[0] && mouse_ndc[0] < end_ndc_cords[0];
            let y_in_window = mouse_ndc[1] > self.ndc_cords[1] && mouse_ndc[1] < end_ndc_cords[1];

            if x_in_window && y_in_window {
                self.pressed = true;
            }
        }

    }

}