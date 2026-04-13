use std::{cell::RefCell, rc::Rc};

use crate::game_data::{World, game_event_manager::prelude::EventManager, player_data::{drone_programming::{function::function_return_value::{ErrorCode, FunctionReturnValue}, var::{game_vars::dynamic_var::DynamicVar, var_type::Var}}, drones::{drone::Drone, drone_actions::{advanced_actions::path_planner::plan_path_to_cords, drone_actions::{DroneAction, DroneActionError}}}, locations::location::WorldLocation}};


#[derive(Clone)]
pub enum DroneAdvancedAction {
    PathToLocation(Option<Rc<RefCell<WorldLocation>>>),
}

impl DroneAdvancedAction {
    pub fn execute(&self, drone: &mut Drone, world: &World) -> FunctionReturnValue {
        match self {
            DroneAdvancedAction::PathToLocation(location_ref_option) => {
                if let Some(location_ref) = location_ref_option {
                    let cords = location_ref.borrow().get_area().get_point_1_cords();
                    plan_path_to_cords(drone, world, cords)
                }
                else {
                    eprintln!("Drone({}) cannot plan path to NULL Location", drone.get_id());
                    return DroneActionError::FailedToPath.wrap();
                }
                
            }
        }
    }

    pub fn wrap_into_action(self) -> DroneAction {
        DroneAction::AdvancedAction(self)
    }

    pub fn get_all_actions() -> Vec<DroneAction> {
        let mut all_actions = Vec::new();
    
        all_actions.push(DroneAdvancedAction::PathToLocation(None).wrap_into_action());
        
        return all_actions;
    }

    //=====================================
    // Identity
    //=====================================

    pub fn get_name(&self) -> String {
        match self {
            DroneAdvancedAction::PathToLocation(location_ref_option) => {
                if let Some(location_ref) = location_ref_option {
                    return format!("Pathing to Location: {}", location_ref.borrow().get_name());
                }
                else {
                    return format!("Pathing to Location: NULL");
                }
                
            },
        }
    }

    //=====================================
    // Execution
    //=====================================

    pub fn create_param_vars(&self) -> Vec<Rc<RefCell<Var>>> {
        let mut params = Vec::new();
        match self {
            DroneAdvancedAction::PathToLocation(location_ref) => {
                params.push(DynamicVar::construct_location_var_ref(location_ref));
            },
        }
        params
    }

    pub fn set_params_from_vars(&mut self, params: &Vec<Rc<RefCell<Var>>>) {
        match self {
            DroneAdvancedAction::PathToLocation(location_ref) => {
                let var_location_option_ref =  DynamicVar::into_location_ref(&params[0]);
                *location_ref = var_location_option_ref;
            },
        }
    }
}