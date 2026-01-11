use std::{collections::HashMap, sync::{Arc, RwLock}};

use miniquad::MouseButton;

use crate::game_data::{TextureManager, World, screen::{Camera, ScreenData, camera_data::{self, CameraData}, drone_ui::drone_ui::{self, DroneUI}, screen_data, text::{self, render_string_at_ndi_cords}}, texture_manager, tik_manager::drones::drone::{self, Drone}, types::DroneUITexture};

/*
#####################
## DroneRenderData ##
#####################
These are structs that contain update information for the screen manager class
*/

struct DroneRenderData {
    id: u32,
    drone_cords: [i32; 3],
    needs_rendering: bool,
    drone_ui: DroneUI,

}

impl DroneRenderData {
    pub fn new(id: u32, drone_cords: [i32; 3]) -> DroneRenderData {
        DroneRenderData {
            id: id,
            drone_cords: drone_cords,
            needs_rendering: false,
            drone_ui: DroneUI::new(),
        }
    }

    pub fn re_render_drone(&self, world: &Arc<RwLock<World>>, camera: &mut Camera, camera_data: &CameraData) {
        // Loop through area around drone
        let drone_casted_tile_cords = camera_data.world_to_casted_tile_cords(self.drone_cords);
        for x_offset in -3..3 {
            for y_offset in -3..3 {
                let casted_cords_to_rerender = [
                    drone_casted_tile_cords[0] + x_offset,
                    drone_casted_tile_cords[1] + y_offset
                ];
                camera.ray_cast_tile_at_cords(world, casted_cords_to_rerender);
            }
        }
    }
}

/*
#######################
## ScreenTaskManager ##
#######################
This file is resposible for managing rendering updates that occer every tik
*/

pub struct DroneRenderingTaskManager {
    drones_render_data: HashMap<u32, DroneRenderData>,

}

impl DroneRenderingTaskManager {
    pub fn new() -> DroneRenderingTaskManager {
        DroneRenderingTaskManager {
            drones_render_data: HashMap::new(),
        }
    }

    //=====================================
    // Drone Updates
    //=====================================

    pub fn add_drone_render_data(&mut self, id: u32, drone_cords: [i32; 3]) {
        self.drones_render_data.insert(id, DroneRenderData::new(id, drone_cords));
    }

    pub fn update_drone_location(&mut self, id: u32, drone_cords: [i32; 3]) {
        if let Some(drone) = self.drones_render_data.get_mut(&id) {
            drone.drone_cords = drone_cords;
            drone.needs_rendering = true;
        }
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn render_drone_movement_to_camera(&mut self, camera: &mut Camera, camera_data: &CameraData, world: Arc<RwLock<World>>, texture_manager: &mut TextureManager){
        // Loop through drones
        for (id, drone_data) in &mut self.drones_render_data.iter_mut() {
            // Re render area around drone if moved 
            if drone_data.needs_rendering {
                drone_data.re_render_drone(&world, camera, &camera_data);
                drone_data.needs_rendering = false;
            }

            // Set window drone NDC cords
            let drone_ndc_cords = camera_data.world_to_ndc_cords(drone_data.drone_cords);
            drone_data.drone_ui.set_drone_world_cords(drone_data.drone_cords);
            drone_data.drone_ui.set_drone_ndc_cords(drone_ndc_cords);
        }
    }

    pub fn render_drone_ui(&mut self, texture_manager: &mut TextureManager, camera_data: &CameraData, world: Arc<RwLock<World>>) {
        let world_gaurd = world.read().unwrap();
        for (id, drone_data) in &mut self.drones_render_data.iter_mut() {
            drone_data.drone_ui.render_drone_ui(texture_manager, camera_data, &world_gaurd);
        }
    }

    //=====================================
    // Controls
    //=====================================
    
    pub fn handle_motion_event(&mut self, screen_data: &ScreenData) {
        for (id, drone_data) in &mut self.drones_render_data.iter_mut() {
            drone_data.drone_ui.handle_motion_event(screen_data);
        }
    }

    pub fn handle_mouse_button_down(&mut self, mouse_button: MouseButton, screen_data: &ScreenData) {
        for (id, drone_data) in &mut self.drones_render_data.iter_mut() {
            drone_data.drone_ui.mouse_button_down_event(mouse_button, screen_data);
        }
    }
    
    //=====================================
    // Execution
    //=====================================

    pub fn execute_render_updates_drone(&mut self, world: Arc<RwLock<World>>, camera: &mut Camera, texture_manager: &mut TextureManager) {
        let camera_data = camera.get_camera_data().clone();

        // Loop through drones
        self.render_drone_movement_to_camera(camera, &camera_data, world.clone(), texture_manager);
        self.render_drone_ui(texture_manager, &camera_data, world);

    }
}