use std::collections::{HashSet, VecDeque};
use crate::game_data::{World, tik_manager::block_updates::block_update_manager::BlockUpdateManager, types::BlockType, world_task_manager::{self, world_task_manager::WorldTaskManager}};


//=====================================
// Leafe
//=====================================

/// Checks if leaf is connected to log directly or through other leaves
/// 
/// ### Why:
/// If a leaf is no longer connected to a log it needs to disapear
/// 
/// ### How:
/// First checks if a log is next to the current leaf and if it's not
/// it checks if naboring leaves have a log next to them until it's checked
/// all leaves or reached max checks
/// 
/// 
fn leaf_connected_to_log(world: &World, start_cords: [i32; 3]) -> bool{
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    
    queue.push_back(start_cords);
    visited.insert(start_cords);
    
    const MAX_CHECKS: usize = 100;
    let mut checks = 0;
    
    while let Some(block_cords) = queue.pop_front() {
        checks += 1;
        if checks > MAX_CHECKS {
            return false; // Too far from any log
        }
        
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if x == 0 && y == 0 && z == 0 { continue; }
                    
                    let cords = [
                        block_cords[0] + x,
                        block_cords[1] + y,
                        block_cords[2] + z,
                    ];
                    
                    if !visited.insert(cords) {
                        continue; // Already checked this block
                    }
                    
                    let scanned_block_type = BlockType::from_id(world.get_world_value(cords));
                    
                    if scanned_block_type == BlockType::BrownTrunk || 
                       scanned_block_type == BlockType::PurpleTrunk {
                        return true;
                    }
                    else if scanned_block_type == BlockType::Leaves {
                        queue.push_back(cords);
                    }
                }
            }
        }
    }
    
    false
}


/// Tik leafe for distruction
pub fn tik_leaf(
    block_update_manager: &mut BlockUpdateManager, 
    world_task_manager: &mut WorldTaskManager, 
    world: &World,
    block_cords: [i32; 3],
    rand_value: u32
) {

    
    // Check if Connected to log
    if leaf_connected_to_log(world, block_cords) {
        
    }
    else {
        if rand_value % 500 == 0{
            world_task_manager.mod_block(block_cords, BlockType::Air.id_as_u16());
            block_update_manager.update_area(world, block_cords);
        }
        else {
            block_update_manager.update_block(block_cords);
        }
    }
}


//=====================================
// Tik Grass
//=====================================

pub fn solid_block_above(world: &World, block_cords: [i32; 3]) -> bool {
    let mut block_above_cords = block_cords;
    block_above_cords[2] += 1;

    if BlockType::from_id(world.get_world_value(block_above_cords)).is_solid() {
        return true;
    }
    else {
        return false;
    }
} 

/// Tik grass for changing to dirt if item is on top
pub fn tik_grass(
    block_update_manager: &mut BlockUpdateManager, 
    world_task_manager: &mut WorldTaskManager, 
    world: &World,
    block_cords: [i32; 3],
    rand_value: u32,
) {
    // If there is a block on top
    if solid_block_above(world, block_cords) {
        world_task_manager.mod_block(block_cords, BlockType::Dirt.id_as_u16());
        block_update_manager.update_block(block_cords)
    }
}

//=====================================
// Tik Dirt
//=====================================

pub fn next_to_block_type(world: &World, block_cords: [i32; 3], block_type: BlockType) -> bool {
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                let cords = [
                    block_cords[0] + x,
                    block_cords[1] + y,
                    block_cords[2] + z,
                ];
                let scanned_block_type = BlockType::from_id(world.get_world_value(cords));
                if scanned_block_type == block_type {
                    return true;
                }
            }
        }
    }
    return false;
}

/// Tik grass for changing to dirt if item is on top
pub fn tik_dirt(
    block_update_manager: &mut BlockUpdateManager, 
    world_task_manager: &mut WorldTaskManager, 
    world: &World,
    block_cords: [i32; 3],
    rand_value: u32,
) {
    if !solid_block_above(world, block_cords) {
        // If it's next to grass try and grow 
        if next_to_block_type(world, block_cords, BlockType::Grass) {
            if rand_value % 500 == 0 { 
                world_task_manager.mod_block(block_cords, BlockType::Grass.id_as_u16());
                block_update_manager.update_area(world, block_cords);
            }
            else {
                block_update_manager.update_block(block_cords);
            }
        }
    }
}