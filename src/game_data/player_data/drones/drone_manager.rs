use core::hash;
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::{Arc, RwLock}};


use crate::game_data::{World, game_event_manager::prelude::EventManager, player_data::drones::drone::Drone};


#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct DroneId {
    id: usize
}

impl DroneId {
    fn new(id: usize) -> DroneId {
        DroneId {
            id
        }
    }

    pub fn as_usize(&self) -> usize {
        self.id
    }
}

pub struct DroneManager{
    current_id: u32,
    drone_map: HashMap::<u32, Rc<RefCell<Drone>>>,
    new_drone_map: HashMap<usize, Drone>,
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
            new_drone_map: HashMap::new(),

        }
    }

    //=====================================
    // Creation
    //=====================================

    // Function for creating a drone
    pub fn create_drone_at_cords(&mut self, cords:[i32; 3]) -> DroneId {
        // Create drone
        let new_id = self.current_id;
        

        // Create the drone
        let drone_id = DroneId::new(new_id as usize);
        let drone = Drone::new(cords, drone_id);
        let drone_ref = Rc::new(RefCell::new(Drone::new(cords, drone_id)));



        // Add drone to hashmap
        self.new_drone_map.insert(drone_id.as_usize(), drone);
        
        self.drone_map.insert(new_id, drone_ref.clone());
        

        println!("Created Drone: ID({})", self.current_id);
        self.current_id += 1; // Update Current Id

        return drone_id;
    }


    //=====================================
    // UI Updating
    //=====================================

    pub fn add_all_drones_to_selection_manager(&mut self) {
        for (key, drone) in self.drone_map.iter() {
            // let widget = VarSource::new(Var::Game(GameVar::Dynamic(DynamicVar::Drone(Some(drone.clone())))));
            // self.selection_panel_update_manager.borrow_mut().add_widget(WidgetType::VarSource(widget));
        }
    }

    //=====================================
    // Tik Managment
    //=====================================
    
    // Tik all the drones in the world
    pub fn tik_drones(&mut self, world: Arc<RwLock<World>>, event_manager: &mut EventManager) {
        let world_gaurd = world.read().unwrap();
        
        // Loop through drones
        self.new_drone_map.retain(|_key, drone| {
            drone.tik_drone(&world_gaurd, event_manager);
            drone.get_health() != 0  // Keep if health > 0
        });

    
    }

    //=====================================
    // Drone Getters
    //=====================================

    pub fn clone_drone_with_id(&self, id: DroneId) -> Option<Drone> {
        if let Some(drone) = self.new_drone_map.get(&id.as_usize()) {
            return Some(drone.clone())
        }
        else {
            None
        }
    }

    pub fn get_drone_with_id_mut(&mut self, id: u32) -> Option<&mut Rc<RefCell<Drone>>> {
        return self.drone_map.get_mut(&id);
    }

    pub fn get_mut_drone(&mut self, id: &DroneId) -> Option<&mut Drone> {
        self.new_drone_map.get_mut(&id.as_usize())
    }
 
    //=====================================
    // Drone Setters
    //=====================================

    pub fn kill_all_drones(&mut self) {
        for (key, drone) in &mut self.drone_map {
            drone.borrow_mut().set_health(0);
        }
    }
}