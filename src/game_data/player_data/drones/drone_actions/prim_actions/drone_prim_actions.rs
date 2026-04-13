use std::{cell::RefCell, rc::Rc};

use crate::game_data::{World, game_event_manager::prelude::EventManager, player_data::{drone_programming::var::var_type::Var, drones::{drone::Drone, drone_actions::{drone_actions::DroneAction, prim_actions::{drone_invintory_actions::DroneInventoryAction, drone_world_actions::DroneWorldAction}}}}};

#[derive(Clone)]
pub enum DronePrimAction {
    DroneWorldAction(DroneWorldAction),
    DroneInventoryAction(DroneInventoryAction),
}

impl DronePrimAction {

    //=====================================
    // Execution
    //=====================================
    pub fn execute(&self, drone: &mut Drone, world: &World, event_manager: &mut EventManager) -> u32 {
        match self {
            DronePrimAction::DroneWorldAction(drone_world_action) => {
                return drone_world_action.execute(drone, world, event_manager)
            },
            DronePrimAction::DroneInventoryAction(drone_inventory_action) => {
                return drone_inventory_action.execute(drone)
            },
        }
    }

    pub fn wrap_into_action(self) -> DroneAction {
        DroneAction::PrimAction(self)
    }

    pub fn get_all_actions() -> Vec<DroneAction> {
        let mut all_actions = Vec::new();
    
        all_actions.append(&mut DroneWorldAction::get_all_actions());
        all_actions.append(&mut DroneInventoryAction::get_all_actions());
        
        return all_actions;
    }

    //=====================================
    // Identity
    //=====================================

    pub fn get_name(&self) -> String {
        match self {
            DronePrimAction::DroneWorldAction(drone_world_action) => {
                return drone_world_action.get_name();
            },
            DronePrimAction::DroneInventoryAction(drone_inventory_action) => {
                return drone_inventory_action.get_name();
            },
        }
    }

    //=====================================
    // Function Construction
    //=====================================
    pub fn create_param_vars(&self) -> Vec<Rc<RefCell<Var>>> {
        match self {
            DronePrimAction::DroneWorldAction(drone_world_action) => {
                drone_world_action.create_param_vars()
            },
            DronePrimAction::DroneInventoryAction(drone_inventory_action) => {
                drone_inventory_action.create_param_vars()
            },
        }
    }

    pub fn set_params_from_vars(&mut self, params: &Vec<Rc<RefCell<Var>>>) {
        match self {
            DronePrimAction::DroneWorldAction(drone_world_action) => {
                drone_world_action.set_params_from_vars(params);
            },
            DronePrimAction::DroneInventoryAction(drone_inventory_action) => {
                drone_inventory_action.set_params_from_vars(params);
            },
        }
    } 

}