use crate::game_data::{World, game_event_manager::prelude::EventManager, player_data::drones::{drone::Drone, drone_actions::prim_actions::drone_prim_actions::DronePrimAction}};

pub enum DroneAction {
    PrimAction(DronePrimAction),
}

impl DroneAction {
    pub fn execute(&self, drone: &mut Drone, world: &World, event_manager: &mut EventManager) {
        match self {
            DroneAction::PrimAction(drone_prim_action) => {
                drone_prim_action.execute(drone, world, event_manager);
            },
        }
    }
}
