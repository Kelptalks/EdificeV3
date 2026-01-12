use std::sync::{Arc, RwLock};

use miniquad::MouseButton;

use crate::game_data::{TextureManager, World, screen::{ScreenData, camera_data::{self, CameraData}, drone_ui::{drone_mini_window::MiniWindow, drone_spectate_window::SpectateWindow}, screen_data}, tik_manager::drones::drone::Drone, types::DroneUITexture};


/*
##################
## Drone Window ##
##################
Controls the rendering an inputs for drone UI
*/


pub struct DroneUI {
    spectate_window: SpectateWindow,
    mini_window: MiniWindow,
}

impl DroneUI {
    pub fn new() -> DroneUI{
        DroneUI {
            mini_window: MiniWindow::new(),
            spectate_window: SpectateWindow::new()
        }
    }
    
    //=====================================
    // Ui Rendering
    //=====================================

    pub fn render_drone_ui(&mut self, texture_manager: &mut TextureManager, camera_data: &CameraData, world: &World, drone: &Drone) {
        self.mini_window.set_scale(camera_data.get_render_scale() * 150.0);
        self.mini_window.render(texture_manager, camera_data, drone);
        if self.spectate_window.is_visible() {
            self.spectate_window.render(texture_manager, world, drone);
        }
    }

    //=====================================
    // Controls
    //=====================================

    pub fn handle_motion_event(&mut self, screen_data: &ScreenData) {
        self.mini_window.handle_motion_event(screen_data);
        if self.spectate_window.is_visible() {
            self.spectate_window.handle_motion_event(screen_data);
        }
    }

    pub fn mouse_button_down_event(&mut self, button: MouseButton, screen_data: &ScreenData) {
        
        if self.spectate_window.is_visible() {
            self.spectate_window.mouse_button_down_event(button, screen_data);
        }
        else {
            self.mini_window.mouse_button_down_event(button, screen_data);
        }
        
        
        
        if self.mini_window.is_pressed() {
            self.spectate_window.set_visible(true);

            // Calcualte the creation cords the window
            let mut spec_window_cords = self.mini_window.get_ndc_cords();
            spec_window_cords[0] += self.mini_window.get_scale();
            self.spectate_window.set_ndc_cords(spec_window_cords);

            self.mini_window.set_pressed(false);
        }
    }


    pub fn handle_mouse_button_up(&mut self, mouse_button_up: MouseButton, screen_data: &ScreenData) {
        if self.spectate_window.is_visible() {
            self.spectate_window.handle_mouse_button_up(mouse_button_up, screen_data);
        }
    }
}
