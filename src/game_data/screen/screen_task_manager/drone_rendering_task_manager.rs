use std::{collections::HashMap, sync::{Arc, RwLock}};

use miniquad::MouseButton;

use crate::game_data::{TextureManager, World, screen::{Camera, ScreenData, camera_data::{self, CameraData}, camera_ui::drone_ui::{self}, screen_data, text::{self, render_string_at_ndc}}, texture_manager, tik_manager::drones::drone::{self, Drone}, types::DroneUITexture};

struct BlockUpdateTask {
    cords: [i32; 3],
}

impl BlockUpdateTask {
    pub fn new(cords: [i32; 3]) -> BlockUpdateTask {
        BlockUpdateTask {
            cords: cords,
        }
    }
    
    pub fn re_render_block(&self, world: &Arc<RwLock<World>>, camera: &mut Camera, camera_data: &CameraData) {
        // Loop through area around drone
        let casted_tile_cords = camera_data.world_to_casted_tile_cords(self.cords);
        camera.ray_cast_area_at_cords(world, casted_tile_cords, 3);
    }
}

/*
#######################
## ScreenTaskManager ##
#######################
This file is resposible for managing rendering updates that occer every tik
*/

pub struct DroneRenderingTaskManager {
    block_update_tasks: Vec<BlockUpdateTask>

}

impl DroneRenderingTaskManager {
    pub fn new() -> DroneRenderingTaskManager {
        DroneRenderingTaskManager {
            block_update_tasks: Vec::new(),
        }
    }

    //=====================================
    // task updates
    //=====================================

    pub fn add_block_render_task(&mut self, cords: [i32; 3]) {
        self.block_update_tasks.push(BlockUpdateTask { cords });
    }

    //=====================================
    // Rendering
    //=====================================


    pub fn render_block_updates_to_camera(&mut self, camera: &mut Camera, camera_data: &CameraData, world: Arc<RwLock<World>>){
        // Loop through drones
        for block_update_task in &mut self.block_update_tasks {
            block_update_task.re_render_block(&world, camera, camera_data);
        }
        // Clear the aray after
        self.block_update_tasks.clear();
    }



    //=====================================
    // Execution
    //=====================================

    pub fn execute_render_updates_drone(&mut self, world: Arc<RwLock<World>>, camera: &mut Camera, texture_manager: &mut TextureManager) {
        let camera_data = camera.get_camera_data().clone();

        // Loop through drones
        self.render_block_updates_to_camera(camera, &camera_data, world.clone())

    }
}