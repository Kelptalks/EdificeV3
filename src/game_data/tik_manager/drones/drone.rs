use std::sync::{Arc, RwLock};

use rand::rand_core::block;

use crate::game_data::{world_task_manager::{self, world_task_manager::WorldTaskManager}};
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
    cords: [i32; 3],
    direction: DroneDirection,
}

impl Drone {
    pub fn new(cords: [i32; 3]) -> Drone {
        Drone {
            cords: cords,
            direction: DroneDirection::ForwardLeft,
        }
    }

    pub fn get_cords(&self) -> [i32; 3] {
        return self.cords;
    }

    pub fn tik_drone(&mut self, world: Arc<RwLock<World>>, world_task_manager: &mut WorldTaskManager) {
        // Get world read lock
        let read_gaurd = world.read().unwrap();
        world_task_manager.mod_block(self.cords, 0); // Clear drone before movement

        // Make drone fall
        let mut block_below_drone = self.cords;
        block_below_drone[2] -= 1;
        let block_bellow_drone = read_gaurd.get_world_value(block_below_drone);
        if block_bellow_drone == BlockType::Air.id_as_u16() {
            self.cords[2] -= 1; // Move drone down one
        }

        world_task_manager.mod_block(self.cords, self.direction.to_block_id()); // Add drone back
    }
}