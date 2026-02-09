use std::collections::HashSet;

use rand::random_range;

use crate::game_data::{World, tik_manager::block_updates::plant_update, types::BlockType, world_task_manager::{self, world_task_manager::WorldTaskManager}};

/*
##########################
## Block Update Manager ##
##########################

*/
pub struct BlockUpdateManager {
    block_update_tasks: HashSet<[i32; 3]>
}

impl BlockUpdateManager {
    pub fn new() -> BlockUpdateManager {
        BlockUpdateManager {
            block_update_tasks: HashSet::new()
        }
    }

    pub fn update_block(&mut self, block_cords: [i32; 3]) {
        self.block_update_tasks.insert(block_cords);
    }

    /// Marks all blocks around block cords as update tasks
    pub fn update_area(&mut self, world: &World, block_cords: [i32; 3]) {
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let cords = [
                        block_cords[0] + x,
                        block_cords[1] + y,
                        block_cords[2] + z,
                    ];
                    let block_type = BlockType::from_id(world.get_world_value(cords));
                    if block_type.is_tikable() {
                        self.block_update_tasks.insert(cords);
                    }
                }
            }
        }
    }

    /// Tik a block in the update manager
    /// 
    /// 
    /// 
    pub fn tik_block(&mut self, 
        world: &World, 
        world_task_manager: &mut WorldTaskManager, 
        block_cords: [i32; 3],
    ) {
        let block_type = BlockType::from_id(world.get_world_value(block_cords));
        let rand_value = random_range(0..1000);
        match block_type {
            BlockType::Air => {
                
            }
            BlockType::Leaves => {
                plant_update::tik_leaf(self, world_task_manager, world, block_cords, rand_value);
            }
            BlockType::Grass =>{
                plant_update::tik_grass(self, world_task_manager, world, block_cords, rand_value);
            }
            BlockType::Dirt => {
                plant_update::tik_dirt(self, world_task_manager, world, block_cords, rand_value);
            }
            _ => {
                
            }
        }
    }

    /// Update blocks for tiking using world task manager mods
    ///
    /// ### Why: 
    /// We need to update all the blocks sarounding blocks modified
    /// during the past tik mostly by drones
    /// 
    /// 
    pub fn update_blocks(&mut self, world: &World, world_task_manager: &mut WorldTaskManager) {
        
        // Update blocks around all the blocks modified
        let blocks_to_add_to_update = world_task_manager.get_blocks_to_update();
        for mod_task in blocks_to_add_to_update {
            self.update_area(world, mod_task.get_cords());
        }
    }


    /// Tik all the blocks in the update manager
    pub fn tik_blocks(&mut self, world: &World, world_task_manager: &mut WorldTaskManager) {
        // Take ownership of the HashSet, leaving a new empty one in its place
        let mut block_update_tasks = std::mem::take(&mut self.block_update_tasks);
        for current_block_cords in block_update_tasks.drain() { 
            self.tik_block(world, world_task_manager, current_block_cords);
        }
    }
}