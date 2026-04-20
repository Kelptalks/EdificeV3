use std::{cell::RefCell, rc::Rc};

use crate::game_data::{World, game_event_manager::prelude::EventManager, player_data::{drone_script::var::{game_vars::dynamic_var::{DynamicVarType, DynamicVarTypeKind}, var::Var, var_type::{VarKind, VarType}}, drones::{drone::Drone, drone_actions::{advanced_actions::path_planner::plan_path_to_cords, drone_actions::{DroneAction, DroneActionError}}}, locations::location::WorldLocation}, texture_manager::texture::Texture, types::UITextures};


#[derive(Clone)]
pub enum DroneAdvancedAction {
    PathToLocation(Option<Rc<RefCell<WorldLocation>>>),
}

impl DroneAdvancedAction {
    pub fn execute(&self, drone: &mut Drone, world: &World) -> Var {
        match self {
            DroneAdvancedAction::PathToLocation(location_ref_option) => {
                if let Some(location_ref) = location_ref_option {
                    let cords = location_ref.borrow().get_area().get_point_1_cords();
                    plan_path_to_cords(drone, world, cords)
                }
                else {
                    eprintln!("Drone({}) cannot plan path to NULL Location", drone.get_id());
                    return DroneActionError::FailedToPath.wrap_into_var_type().create_var();
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
                "PathToLocation".to_string()
                
            },
        }
    }

    pub fn get_texture(&self) -> Texture {
        match self {
            DroneAdvancedAction::PathToLocation(ref_cell) => {
                UITextures::DroneActionPathIcon.wrap_into_texture()
            },
        }
    }

    //=====================================
    // Execution
    //=====================================

    pub fn get_param_var_kinds(&self) -> Vec<VarKind> {
        let mut params = Vec::new();
        match self {
            DroneAdvancedAction::PathToLocation(_location_ref) => {
                params.push(DynamicVarTypeKind::Location.wrap_into_var_kind());
            },
        }
        params
    }

    pub fn set_params_from_vars(&mut self, params: &Vec<Rc<RefCell<VarType>>>) {
        match self {
            DroneAdvancedAction::PathToLocation(location_ref) => {
                let var_location_option_ref =  DynamicVarType::into_location_ref(&params[0]);
                *location_ref = var_location_option_ref;
            },
        }
    }
    
    
}