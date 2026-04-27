use crate::game_data::{game_event_manager::prelude::{Event, EventManager}, player_data::drones::{drone::Drone, drone_manager::DroneId}};

pub struct DroneEventScheduler {
    drone_clone: Drone,
    events: Vec<Event>
}

impl DroneEventScheduler {
    pub fn new(drone_clone: Drone) -> DroneEventScheduler {
        DroneEventScheduler {
            drone_clone,
            events: Vec::new(),
        }
    }

    

    pub fn schedul_events(&self, event_manager: &mut EventManager) {

    }
}