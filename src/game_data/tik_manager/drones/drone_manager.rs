use std::{collections::HashMap, process::id, sync::{Arc, RwLock}};

use rand::distr::Map;

use crate::game_data::{World, screen::screen_task_manager::{self, screen_task_manager::ScreenTaskManager}, tik_manager::drones::{drone::Drone, lua_manager::LuaManager}, world_task_manager::world_task_manager::WorldTaskManager};

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

    pub fn get_all_drone_ids(&mut self) -> Vec<u32> {
        let mut drone_ids: Vec<u32> = Vec::new();
        for (key, drone) in &mut self.drone_map {
            drone_ids.push(drone.get_id());
        }
        return drone_ids;
    }

    pub fn get_drone_with_id(&mut self, id: u32) -> Option<&mut Drone> {
        return self.drone_map.get_mut(&id);
    }

    // Tik all the drones in the world
    pub fn tik_drones(&mut self, world: Arc<RwLock<World>>, world_task_manager: &mut WorldTaskManager, screen_task_manager: &mut ScreenTaskManager) {
        let world_gaurd = world.read().unwrap();
        
        for (key, drone) in &mut self.drone_map {
            drone.tik_drone(&world_gaurd, world_task_manager);
            screen_task_manager.add_drone_render_task(drone.get_cords());
        }
    
    }

    // Function for creating a drone
    pub fn create_drone_at_cords(&mut self, cords:[i32; 3], name: String){
        // Create drone
        let new_id = self.current_id;
        self.current_id += 1; // Update Current Id

        // Create the drone
        let drone = Drone::new(cords, name.clone(), new_id);

        // Add drone to hashmap
        self.drone_map.insert(new_id, drone);

        // Debug
        println!("Created New Drone | Cords: ({:?}) | Id: {} | Name: {}", cords, new_id, name);

    }
}