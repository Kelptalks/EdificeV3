use std::sync::{Arc, RwLock};

use crate::game_data::{World, screen::Camera};

/*
####################
## Tasks ##
####################
These are structs that contain update information for the screen manager class
*/
struct DroneRenderTask {
    drone_cords: [i32; 3],
    
}

impl DroneRenderTask {
    pub fn new(drone_cords: [i32; 3]) -> DroneRenderTask {
        DroneRenderTask {
            drone_cords
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
    drone_render_tasks: Vec<DroneRenderTask>,

}

impl ScreenTaskManager {
    pub fn new() -> ScreenTaskManager {
        ScreenTaskManager {
            drone_render_tasks: Vec::new(),
        }
    }

    pub fn add_drone_render_task(&mut self, drone_cords: [i32; 3]) {
        self.drone_render_tasks.push(DroneRenderTask::new(drone_cords));
    }

    pub fn execute_tasks(&mut self, world: Arc<RwLock<World>>, camera: &mut Camera) {
        let camera_data = camera.get_camera_data().clone();

        for task in &mut self.drone_render_tasks {
            // Loop through area around drone
            let drone_casted_tile_cords = camera_data.world_to_casted_tile_cords(task.drone_cords);
            for x_offset in -1..2 {
                for y_offset in -1..2 {
                    let casted_cords_to_rerender = [
                        drone_casted_tile_cords[0] + x_offset,
                        drone_casted_tile_cords[1] + y_offset
                    ];
                    camera.ray_cast_tile_at_cords(&world, casted_cords_to_rerender);

                }
            }
        }

        self.drone_render_tasks.clear();
    }
}