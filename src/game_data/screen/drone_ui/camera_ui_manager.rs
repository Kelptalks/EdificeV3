use std::{collections::HashMap, sync::{Arc, RwLock}};

use miniquad::MouseButton;

use crate::game_data::{TextureManager, World, screen::{Button, ScreenData, camera_data::CameraData, drone_ui::drone_ui::DroneUI, renderer::camera}, tik_manager::{self, tik_manager::TikManager}};

pub struct CameraUIManager {
    drone_ui_map: HashMap<u32, DroneUI>,
    toggle_drone_ui: Button,
    drone_ui_visible: bool,
}

impl CameraUIManager {
    pub fn new() -> CameraUIManager {
        CameraUIManager {
            drone_ui_map: HashMap::new(),
            toggle_drone_ui: Button::new([-0.99, 0.0], 0.07, crate::game_data::types::UITextures::ButtonCircle),
            drone_ui_visible: true,
        }
    }

    pub fn render_ui(&mut self, texture_manager: &mut TextureManager, camera_data: &CameraData, world: &Arc<RwLock<World>>, tik_manager: &TikManager) {
        self.toggle_drone_ui.render_button(texture_manager);
        self.toggle_drone_ui.render_block_on_button(texture_manager, crate::game_data::types::BlockType::DroneBotRight);
        if !self.drone_ui_visible {
            return;
        }

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
        self.toggle_drone_ui.handle_mouse_motion_input(screen_data);
        for (id, drone_ui) in &mut self.drone_ui_map.iter_mut() {
            drone_ui.handle_motion_event(screen_data);
        }
    }

    pub fn handle_mouse_button_down(&mut self, mouse_button: MouseButton, screen_data: &ScreenData) {
        if mouse_button == MouseButton::Left {
            if self.toggle_drone_ui.is_mouse_on_button() {
                self.drone_ui_visible = !self.drone_ui_visible;
            }
        }

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