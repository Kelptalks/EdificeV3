use std::{cell::RefCell, rc::Rc};

use crate::game_data::{World, game_event_manager::prelude::EventManager, player_data::{drone_programming::var::var_type::Var, drones::{drone::Drone, drone_actions::prim_actions::drone_prim_actions::DronePrimAction}}};

#[derive(Clone)]
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

    pub fn create_param_vars(&self) -> Vec<Rc<RefCell<Var>>> {
        match self {
            DroneAction::PrimAction(drone_prim_action) => {
                drone_prim_action.create_param_vars()
            },
        }
    }

    pub fn set_params_from_vars(&mut self, params: &Vec<Rc<RefCell<Var>>>) {
        match self {
            DroneAction::PrimAction(drone_prim_action) => {
                drone_prim_action.set_params_from_vars(params);
            },
        }
    }
}
