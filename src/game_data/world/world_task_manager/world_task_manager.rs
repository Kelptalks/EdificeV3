use std::sync::{Arc, RwLock};

use crate::game_data::{World, screen::screen_task_manager::{self, drone_rendering_task_manager::{self, DroneRenderingTaskManager}}};

struct ModBlockTask {
    world_cords : [i32; 3],
    block_type : u16,
}
impl ModBlockTask {
    pub fn new(world_cords : [i32; 3], block_type: u16) -> Self {
        ModBlockTask {
            world_cords,
            block_type,
        }
    }
}

/*
########################
## World Task Manager ##
########################
This is the manager for minipulating the world in a single write lock.
*/


pub struct WorldTaskManager {
    block_modding_tasks : Vec<ModBlockTask>, // A set of tasks to modify a block
    
}

impl WorldTaskManager {
    pub fn new() -> Self {
        WorldTaskManager {
            block_modding_tasks : Vec::new(),
        }
    }

    // Add a mod block task to the manager
    pub fn mod_block(&mut self, world_cords : [i32; 3], block_type: u16) {
        self.block_modding_tasks.push(ModBlockTask::new(world_cords, block_type));
    }

    // Execute the tasks in the added to the manager
    pub fn execute_tasks(&mut self, world: Arc<RwLock<World>>, screen_task_manager: &mut DroneRenderingTaskManager) {
         // get the write lock of the world
        let mut world_gaurd = world.write().unwrap();

        for task in &mut self.block_modding_tasks {
            world_gaurd.set_world_value(task.block_type, task.world_cords);
            screen_task_manager.add_block_render_task(task.world_cords); // Re render the block modified
        }
        self.block_modding_tasks.clear(); // Clear the list for the next task execution window


    }

}