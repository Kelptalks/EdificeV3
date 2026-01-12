use std::{collections::HashMap, sync::{Arc, RwLock}};

use miniquad::MouseButton;

use crate::game_data::{TextureManager, World, screen::{ScreenData, camera_data::CameraData, drone_ui::drone_ui::DroneUI, renderer::camera}, tik_manager::{self, tik_manager::TikManager}};

pub struct DroneUIManager {
    drone_ui_map: HashMap<u32, DroneUI>,
}

impl DroneUIManager {
    pub fn new() -> DroneUIManager {
        DroneUIManager {
            drone_ui_map: HashMap::new(),
        }
    }

    pub fn render_ui(&mut self, texture_manager: &mut TextureManager, camera_data: &CameraData, world: &Arc<RwLock<World>>, tik_manager: &TikManager) {
        
        let drone_manager = tik_manager.get_drone_manager();
        let drone_ids = drone_manager.get_all_drone_ids();
        
        let world_gaurd = world.read().unwrap();
        for drone_id in drone_ids {
            if let Some(drone_ui) = self.drone_ui_map.get_mut(&drone_id) {
                if let Some(drone) = drone_manager.get_drone_with_id(drone_id){
                    drone_ui.render_drone_ui(texture_manager, camera_data, &world_gaurd, drone);
                }
            }
            else {
                self.drone_ui_map.insert(drone_id, DroneUI::new());
            }
        }
    }


    //=====================================
    // Controls
    //=====================================
    
    pub fn handle_motion_event(&mut self, screen_data: &ScreenData) {
        for (id, drone_ui) in &mut self.drone_ui_map.iter_mut() {
            drone_ui.handle_motion_event(screen_data);
        }
    }

    pub fn handle_mouse_button_down(&mut self, mouse_button: MouseButton, screen_data: &ScreenData) {
        for (id, drone_ui) in &mut self.drone_ui_map.iter_mut() {
            drone_ui.mouse_button_down_event(mouse_button, screen_data);
        }
    }
    
    pub fn handle_mouse_button_up(&mut self, mouse_button_up: MouseButton, screen_data: &ScreenData) {
        for (id, drone_ui) in &mut self.drone_ui_map.iter_mut() {
            drone_ui.handle_mouse_button_up(mouse_button_up, screen_data);
        }
    }

    
}