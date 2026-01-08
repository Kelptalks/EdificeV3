use std::{collections::HashMap, sync::{Arc, RwLock}};

use crate::game_data::{TextureManager, World, screen::{Camera, camera_data::{self, CameraData}, render_centered_string_at_ndi_cords, text::{self, render_string_at_ndi_cords}}, texture_manager, tik_manager::drones::drone::{self, Drone}, types::DroneUITexture};

/*
####################
## Tasks ##
####################
These are structs that contain update information for the screen manager class
*/
struct DroneRenderData {
    id: u32,
    drone_cords: [i32; 3],

    needs_rendering: bool,
}

impl DroneRenderData {
    pub fn new(id: u32, drone_cords: [i32; 3]) -> DroneRenderData {
        DroneRenderData {
            id: id,
            drone_cords: drone_cords,
            needs_rendering: false,
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
####################
## ScreenTaskManager ##
####################
This file is resposible for managing rendering updates that occer every tik
*/

pub struct ScreenTaskManager {
    drones_render_data: HashMap<u32, DroneRenderData>,

}

impl ScreenTaskManager {
    pub fn new() -> ScreenTaskManager {
        ScreenTaskManager {
            drones_render_data: HashMap::new(),
        }
    }

    pub fn add_drone_render_data(&mut self, id: u32, drone_cords: [i32; 3]) {
        self.drones_render_data.insert(id, DroneRenderData::new(id, drone_cords));
    }

    pub fn update_drone_location(&mut self, id: u32, drone_cords: [i32; 3]) {
        if let Some(drone) = self.drones_render_data.get_mut(&id) {
            drone.drone_cords = drone_cords;
            drone.needs_rendering = true;
        }
    }

    pub fn execute_render_updates_drone(&mut self, world: Arc<RwLock<World>>, camera: &mut Camera, texture_manager: &mut TextureManager) {
        let camera_data = camera.get_camera_data().clone();

        // Loop through drones
        for (id, drone_data) in &mut self.drones_render_data.iter_mut() {
            // Re render area around drone if moved 
            if drone_data.needs_rendering {
                drone_data.re_render_drone(&world, camera, &camera_data);
                drone_data.needs_rendering = false;
            }

            // Render Drone UI
            let mut drone_ndc_cords = camera_data.world_to_ndc_cords(drone_data.drone_cords);
            let scale =  camera_data.get_render_scale() * 150.0;
            
            let y_offset = scale / 3.0;
            drone_ndc_cords[1] -= y_offset;
            texture_manager.render_drone_ui_element_centered(DroneUITexture::DroneMiniWindow, drone_ndc_cords, scale);
        }
    }
}