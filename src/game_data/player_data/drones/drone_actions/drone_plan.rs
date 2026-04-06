use crate::game_data::{World, game_event_manager::prelude::EventManager, player_data::drones::{drone::Drone, drone_actions::drone_actions::DroneAction}};

pub struct DronePlan {
    actions: Vec<DroneAction>,

    completed: bool,
    failed: bool,
}


impl DronePlan {
    pub fn new() -> DronePlan {
        DronePlan {
            actions: Vec::new(), 
            
            completed: false, 
            failed: false,
        }
    }

    pub fn add_action(&mut self, action: DroneAction) {
        self.actions.insert(0, action);
    }

  
    pub fn pop_next_action(&mut self) -> Option<DroneAction>{
        // If this is the last action left mark as completed
        if self.actions.is_empty() {
            self.completed = true;
        }
        self.actions.pop()
    }


    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn failed(&mut self) {
        self.failed = true;
    }

    pub fn has_failed(&self) -> bool {
        self.failed
    }
}