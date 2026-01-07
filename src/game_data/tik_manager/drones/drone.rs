use std::sync::{Arc, RwLock};

use rand::rand_core::block;

use crate::game_data::{world_task_manager::{world_task_manager::WorldTaskManager}};
use crate::game_data::{types::BlockType, World};

pub enum DroneDirection {
    ForwardLeft,
    ForwardRight,
    BackLeft,
    BackRight,
}

impl DroneDirection {
    pub fn to_block_id(&self) -> u16 {
        match self {
            DroneDirection::ForwardLeft => 51,
            DroneDirection::ForwardRight => 52,
            DroneDirection::BackLeft => 53,
            DroneDirection::BackRight => 54,
        }
    }

}

pub struct Drone{
    // identity
    name: String,
    id: u32,

    // Position
    cords: [i32; 3],
    direction: DroneDirection,

    // Stats
    busy_time: u16,
    fuel: u32,
    vision_range: u16,
    modify_range: u16,
    mine_power: u16,
    chop_power: u16,



    
}

impl Drone {
    pub fn new(cords: [i32; 3], name: String, id: u32) -> Drone {
        Drone {
            // identity
            name: name,
            id: id,

            // Position
            cords: cords,
            direction: DroneDirection::ForwardLeft,

            // Stats
            busy_time: 0,
            fuel: 10000,
            vision_range: 2,
            modify_range: 1,
            mine_power: 1,
            chop_power: 1,
        }
    }

    //=====================================
    // Getters
    //=====================================

    fn get_relative_world_cords(&self, relative_cords: [i32; 3]) -> [i32; 3] {
        // Calculate the world cords based of drone position 
        let drone_cords = self.cords;
        let world_cords = [
            drone_cords[0] + relative_cords[0], 
            drone_cords[1] + relative_cords[1], 
            drone_cords[2] + relative_cords[2]
        ];
        return world_cords;
    }

    fn if_cords_within_range(relative_cords: [i32; 3], range: u16) -> bool {
        for i in relative_cords {
            if i.abs() as u16 > range {
                return false;
            }
        }
        return true;
    }

    //=====================================
    // Drone Getters
    //=====================================

    pub fn get_cords(&self) -> [i32; 3] {
        return self.cords;
    }

    pub fn get_id(&self) -> u32 {
        return self.id;
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn is_busy(&self) -> bool {
        return self.busy_time != 0;
    }

    // Get a block based of cords relative to the drone
    pub fn scan_block(&self, world: &World, relative_cords: [i32; 3]) -> BlockType {
        // check if scan is in vision range
        if Drone::if_cords_within_range(relative_cords, self.vision_range){
            // Calculate the world cords based of drone position 
            let world_cords = self.get_relative_world_cords(relative_cords);

            // Get block from world
            let block = BlockType::from_id(world.get_world_value(world_cords));
            return block;
        }
        else {
            return BlockType::Debug;
        }
    }

    //=====================================
    // Actions
    //=====================================


    pub fn move_drone(&mut self, world: &World, relative_cords: [i32; 3], world_task_manager: &mut WorldTaskManager){
        if self.is_busy() {
            println!("Drone {} cannot move because busy", self.id);
            return;
        }
        
        if Drone::if_cords_within_range(relative_cords, 1){
            let world_cords = self.get_relative_world_cords(relative_cords);
            let block_type_of_new_location = BlockType::from_id(world.get_world_value(world_cords));

            // If block is not solid allow movment
            if !block_type_of_new_location.is_solid() {
                // Get block bellow to calculate move speed
                let cords_below_drone = self.get_relative_world_cords([0, 0, -1]);
                let block_below_drone = BlockType::from_id(world.get_world_value(cords_below_drone));

                // Don't allow movement if falling
                if block_below_drone == BlockType::Air {
                    return;
                }

                // Update drones cords
                for i in 0..3{
                    world_task_manager.mod_block(self.cords, 0); // Clear drone in old location
                    self.cords[i] += relative_cords[i];
                    world_task_manager.mod_block(self.cords, self.direction.to_block_id()); // Add drone back in new location 
                }
                // Set drone busy time
                self.busy_time += block_below_drone.friction();
            }
        }
    }

    // Mine a block relative to the drone
    pub fn mine_block(&mut self, relative_cords: [i32; 3], world: &World, world_task_manager: &mut WorldTaskManager) {
        if self.is_busy() {
            println!("Drone {} cannot mine because busy", self.id);
            return;
        }
        
        // check if scan is in mine range
        if Drone::if_cords_within_range(relative_cords, self.modify_range) {
            // Get world cords
            let world_cords = self.get_relative_world_cords(relative_cords);
            let block_to_mine = BlockType::from_id(world.get_world_value(world_cords));


            // Change the block to air
            world_task_manager.mod_block(world_cords, BlockType::Air.id_as_u16());
            self.busy_time += block_to_mine.hardness();
        }
    }


    // Place a block relative to the drone
    pub fn place_block(&mut self, relative_cords: [i32; 3], world_task_manager: &mut WorldTaskManager, block: BlockType) {
        if self.is_busy() {
            println!("Drone {} cannot mine because busy", self.id);
            return;
        }
        
        if Drone::if_cords_within_range(relative_cords, self.modify_range) {
            // Get world cords
            let world_cords = self.get_relative_world_cords(relative_cords);

            // Change the block to air
            world_task_manager.mod_block(world_cords, block.id_as_u16());
        }
    }


    //=====================================
    // Tikking
    //=====================================

    pub fn tik_drone(&mut self, world: &World, world_task_manager: &mut WorldTaskManager) {
        if self.is_busy() {
            self.busy_time -= 1;
            self.fuel -= 1;
            return;
        }
        else if self.fuel <= 0 {
            return;
        }
        // Ready for next action
        else {
            // Get world read lock
            world_task_manager.mod_block(self.cords, 0); // Clear drone before movement

            // Make drone fall of no solid blocks below
            let mut block_below_drone = self.cords;
            block_below_drone[2] -= 1;
            let block_bellow_drone = BlockType::from_id(world.get_world_value(block_below_drone));
            if !block_bellow_drone.is_solid() {
                self.cords[2] -= 1; // Move drone down one
            }

            world_task_manager.mod_block(self.cords, self.direction.to_block_id()); // Add drone back
        }
    }
}