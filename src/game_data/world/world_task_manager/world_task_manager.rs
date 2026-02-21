use std::sync::{Arc, RwLock};

use crate::game_data::{World, screen::screen_task_manager::{self, rendering_task_manager::{self, RenderingTaskManager}}, types::BlockTexture};

pub struct ModBlockTask {
    world_cords : [i32; 3],
    block_type : u16,
    original_block_type: u16,
}
impl ModBlockTask {
    pub fn new(world_cords : [i32; 3], block_type: u16) -> Self {
        ModBlockTask {
            world_cords: world_cords,
            block_type: block_type,
            original_block_type: 0,
        }
    }

    //=====================================
    // Getters
    //=====================================
    
    pub fn get_cords(&self) -> [i32; 3] {
        return self.world_cords;
    } 

    pub fn get_block_type(&self) -> BlockTexture {
        return BlockTexture::from_id(self.block_type);
    }
}

/*
########################
## World Task Manager ##
########################
This is the manager for minipulating the world

Why :
    To modify the world with one write lock in one centrilized location
    and to control the order in which blocks are modified

*/


pub struct WorldTaskManager {
    completed_block_modding_tasks : Vec<ModBlockTask>,
    block_modding_tasks : Vec<ModBlockTask>, // A set of tasks to modify a block

    undo_all_tasks: bool,
}

impl WorldTaskManager {
    pub fn new() -> Self {
        WorldTaskManager {
            completed_block_modding_tasks : Vec::new(),
            block_modding_tasks : Vec::new(),

            undo_all_tasks: false,
        }
    }

    //=====================================
    // Task adding
    //=====================================

    // Add a mod block task to the manager
    pub fn mod_block(&mut self, world_cords : [i32; 3], block_type: u16) {
        self.block_modding_tasks.push(ModBlockTask::new(world_cords, block_type));
    }

    //=====================================
    // Getters
    //=====================================

    pub fn get_blocks_to_update(&self) -> &Vec<ModBlockTask>{
        return &self.block_modding_tasks;
    }


    //=====================================
    // Execution
    //=====================================

    // Execute the tasks in the added to the manager
    pub fn execute_tasks(&mut self, world: Arc<RwLock<World>>, screen_task_manager: &mut RenderingTaskManager) {
        // Get the write lock of the world
        let mut world_guard = match world.write() {
            Ok(guard) => guard,
            Err(poisoned) => {
                // Lock was poisoned, but we can still access the data
                eprintln!("Warning: World lock was poisoned, recovering...");
                poisoned.into_inner()
            }
        };

        for task in &mut self.block_modding_tasks {
            // Save old world value. 
            task.original_block_type = world_guard.get_world_value(task.world_cords);

            // Set new world value
            world_guard.set_world_value(task.block_type, task.world_cords);
            screen_task_manager.add_block_render_task(task.world_cords);
        }

        // Move completed tasks to the completed array
        self.completed_block_modding_tasks.append(&mut self.block_modding_tasks);
        // block_modding_tasks is now empty after append

        // Undo all tasks if needed
        if self.undo_all_tasks {
            self.execute_undo_all_tasks(&mut world_guard, screen_task_manager)
        }
    }

    pub fn execute_undo_all_tasks(&mut self, world: &mut World, screen_task_manager: &mut RenderingTaskManager) {
        println!("total tasks: {}", self.completed_block_modding_tasks.len());
        // Undo in reverse order, applying directly without creating new tasks
        while let Some(task) = self.completed_block_modding_tasks.pop() {
            world.set_world_value(task.original_block_type, task.world_cords);
            screen_task_manager.add_block_render_task(task.world_cords);
        }
        self.undo_all_tasks = false;
    }

    pub fn undo_all_tasks(&mut self) {
        self.undo_all_tasks = true;
    }

}