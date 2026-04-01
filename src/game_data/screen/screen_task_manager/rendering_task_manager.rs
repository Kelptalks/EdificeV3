

use crate::game_data::{TextureManager, screen::{Camera, camera_data::CameraData}};

struct BlockUpdateTask {
    cords: [i32; 3],
}

impl BlockUpdateTask {
    pub fn new(cords: [i32; 3]) -> BlockUpdateTask {
        BlockUpdateTask {
            cords: cords,
        }
    }
    
    pub fn re_render_block(&self, camera: &mut Camera, camera_data: &CameraData) {
        // Loop through area around drone
        let casted_tile_cords = camera_data.world_to_casted_tile_cords(self.cords);
        camera.dirty_tiles_in_area(casted_tile_cords, 2);
    }
}

/*
#######################
## ScreenTaskManager ##
#######################
This file is resposible for managing rendering updates that occer every tik
*/

pub struct RenderingTaskManager {
    // Tasks
    block_update_tasks: Vec<BlockUpdateTask>,
}

impl RenderingTaskManager {
    pub fn new() -> RenderingTaskManager {
        RenderingTaskManager {
            // Tasks
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


    pub fn render_block_updates_to_camera(&mut self, camera: &mut Camera, camera_data: &CameraData){
        // Loop through drones
        for block_update_task in &mut self.block_update_tasks {
            block_update_task.re_render_block(camera, camera_data);
        }
        // Clear the aray after
        self.block_update_tasks.clear();
    }



    //=====================================
    // Execution
    //=====================================

    pub fn execute_render_updates_drone(&mut self, camera: &mut Camera, texture_manager: &mut TextureManager) {
        let camera_data = camera.get_camera_data().clone();

        // Loop through drones
        self.render_block_updates_to_camera(camera, &camera_data)
    }
}