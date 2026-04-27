use crate::game_data::{game_event_manager::prelude::{Event, EventManager, PlayerDataEvent}, player_data::{drones::{drone::Drone, drone_actions::drone_actions::DroneAction, drone_manager::DroneId}, player_data::PlayerData}};

#[derive(Clone)]
pub enum NewDroneEvent {
    AddAction(DroneAction),
}

impl NewDroneEvent {
    pub fn wrap_into_event(self, drone_id: DroneId) -> Event {
        PlayerDataEvent::DroneEventNew(drone_id, self).wrap_into_event()
    }

    pub fn execute(&self, drone_id: &DroneId, player_data: &mut PlayerData) {
        match self {
            NewDroneEvent::AddAction(drone_action) => {
                let drone = player_data.get_mut_drone_manager().get_mut_drone(drone_id);
                if let Some(drone) = drone {
                    drone.add_action(drone_action.clone());
                }
            },
        }

    }
}

pub struct DroneEventScheduler {
    drone_clone: Drone,
    events: Vec<NewDroneEvent>
}

impl DroneEventScheduler {
    pub fn new(drone_clone: Drone) -> DroneEventScheduler {
        DroneEventScheduler {
            drone_clone,
            events: Vec::new(),
        }
    }

    pub fn get_drone(&self) -> &Drone {
        &self.drone_clone
    } 
    
    pub fn give_action(&mut self, action: DroneAction) {
        self.events.push(NewDroneEvent::AddAction(action));
    }

    pub fn schedul_events(&mut self, event_manager: &mut EventManager) {
        let drone_id = self.drone_clone.get_id();

        while let Some(drone_event) = self.events.pop() {
            event_manager.add_event(drone_event.wrap_into_event(drone_id));
        }
    }
}