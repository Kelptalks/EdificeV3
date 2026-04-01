use std::{collections::HashMap, sync::{Arc, RwLock}};


use crate::game_data::{World, tik_manager::drones::drone::Drone, world_task_manager::world_task_manager::WorldTaskManager};

pub struct DroneManager{
    current_id: u32,
    drone_map: HashMap::<u32, Drone>,
}

/*
####################
## Drone Manager ##
####################
Handles all drones 
*/


impl DroneManager {
    pub fn new() -> DroneManager{
        DroneManager { 
            current_id: 0,
            drone_map: HashMap::new(),
        }
    }

    // I should add this as a cashed array 
    pub fn get_all_drone_ids(&self) -> Vec<u32> {
        let mut drone_ids: Vec<u32> = Vec::new();
        for (key, drone) in &self.drone_map {
            drone_ids.push(drone.get_id());
        }
        return drone_ids;
    }



    pub fn get_drone_with_id(&self, id: u32) -> Option<&Drone> {
        return self.drone_map.get(&id);
    }

    pub fn get_drone_with_id_mut(&mut self, id: u32) -> Option<&mut Drone> {
        return self.drone_map.get_mut(&id);
    }

    // Tik all the drones in the world
    pub fn tik_drones(&mut self, world: Arc<RwLock<World>>, world_task_manager: &mut WorldTaskManager) {
        let world_gaurd = world.read().unwrap();
        
        // Loop through drones
        self.drone_map.retain(|_key, drone| {
            drone.tik_drone(&world_gaurd, world_task_manager);
            drone.get_health() != 0  // Keep if health > 0
        });

    
    }

    // Function for creating a drone
    pub fn create_drone_at_cords(&mut self, cords:[i32; 3]){
        // Create drone
        let new_id = self.current_id;
        self.current_id += 1; // Update Current Id

        // Create the drone
        let drone = Drone::new(cords, new_id);

        // Add drone to hashmap
        self.drone_map.insert(new_id, drone);

    }

    pub fn kill_all_drones(&mut self) {
        for (key, drone) in &mut self.drone_map {
            drone.set_health(0);
        }
    }
}