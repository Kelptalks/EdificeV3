use crate::game_data::{World, game_event_manager::prelude::EventManager, player_data::drones::{drone::Drone, drone_actions::prim_actions::{drone_invintory_actions::DroneInventoryAction, drone_world_actions::DroneWorldAction}}};

pub enum DronePrimAction {
    DroneWorldAction(DroneWorldAction),
    DroneInventoryAction(DroneInventoryAction),
}

impl DronePrimAction {
    pub fn execute(&self, drone: &mut Drone, world: &World, event_manager: &mut EventManager) {
        match self {
            DronePrimAction::DroneWorldAction(drone_world_action) => {
                drone_world_action.execute(drone, world, event_manager);
            },
            DronePrimAction::DroneInventoryAction(drone_inventory_action) => {
                drone_inventory_action.execute(drone);
            },
        }
    }
}