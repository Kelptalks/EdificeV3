use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::{Arc, RwLock}};


use crate::game_data::{World, game_event_manager::prelude::EventManager, player_data::drones::drone::Drone};


#[derive(Clone, Copy)]
pub struct DroneId {
    id: usize
}

impl DroneId {
    pub fn as_usize(&self) -> usize {
        self.id
    }
}

pub struct DroneManager{
    current_id: u32,
    drone_map: HashMap::<u32, Rc<RefCell<Drone>>>,

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

    //=====================================
    // Creation
    //=====================================

    // Function for creating a drone
    pub fn create_drone_at_cords(&mut self, cords:[i32; 3]) -> Rc<RefCell<Drone>>{
        // Create drone
        let new_id = self.current_id;
        

        // Create the drone
        let drone = Rc::new(RefCell::new(Drone::new(cords, new_id)));

        // Add the drone to the selection menu
        //let widget = VarSource::new(Var::Game(GameVar::Dynamic(DynamicVar::Drone(Some(drone.clone())))));
        // self.selection_panel_update_manager.borrow_mut().add_widget(WidgetType::VarSource(widget));

        // Add drone to hashmap
        self.drone_map.insert(new_id, drone.clone());
        

        println!("Created Drone: ID({})", self.current_id);
        self.current_id += 1; // Update Current Id

        return drone;
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
        self.drone_map.retain(|_key, drone| {
            drone.borrow_mut().tik_drone(&world_gaurd, event_manager);
            drone.borrow().get_health() != 0  // Keep if health > 0
        });

    
    }

    //=====================================
    // Drone Getters
    //=====================================

    pub fn clone_drone_with_id(&self, id: DroneId) -> Option<Drone> {
        let id = id.as_usize() as u32;
        if let Some(drone) = self.drone_map.get(&id) {
            return Some(drone.borrow().clone())
        }
        else {
            None
        }
    }

    pub fn get_drone_with_id_mut(&mut self, id: u32) -> Option<&mut Rc<RefCell<Drone>>> {
        return self.drone_map.get_mut(&id);
    }
    
    // I should add this as a cashed array 
    pub fn get_all_drone_ids(&self) -> Vec<u32> {
        let mut drone_ids: Vec<u32> = Vec::new();
        for (key, drone) in &self.drone_map {
            drone_ids.push(drone.borrow().get_id());
        }
        return drone_ids;
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