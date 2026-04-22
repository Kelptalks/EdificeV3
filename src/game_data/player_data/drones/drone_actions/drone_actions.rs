use std::{cell::RefCell, rc::Rc};

use crate::game_data::{World, game_event_manager::prelude::EventManager, player_data::{drone_script::{action::action_type::ActionType, var::{game_vars::action_var::{ActionVarType, ErrorCode}, var::Var, var_type::{VarKind, VarType}}}, drones::{drone::Drone, drone_actions::{advanced_actions::advanced_drone_actions::DroneAdvancedAction, getter_actions::getter_actions::DroneGetterAction, prim_actions::drone_prim_actions::DronePrimAction}}}, texture_manager::texture::Texture};



#[derive(Clone, PartialEq)]
pub enum DroneActionError {
    Ok,
    Busy,
    FailedToPath,
    OutOfRange,
    BlockInWay,
    Falling,
    CannotPiller,
    MissingItem,
    UncraftableItem,
    MissingSlot,
}


impl DroneActionError {
    pub fn wrap_into_var_type(self) -> VarType {
        ActionVarType::Status(ErrorCode::DroneActionError(self)).wrap_into_var_type()
    }

    pub fn get_name(&self) -> String {
        match self {
            DroneActionError::Ok => "Ok".to_string(),
            DroneActionError::Busy => "busy".to_string(),
            DroneActionError::FailedToPath => "FailedToPath".to_string(),
            DroneActionError::OutOfRange => "OutOfRange".to_string(),
            DroneActionError::BlockInWay => "BlockInWay".to_string(),
            DroneActionError::Falling => "Falling".to_string(),
            DroneActionError::CannotPiller => "CannotPiller".to_string(),
            DroneActionError::MissingItem => "MissingItem".to_string(),
            DroneActionError::UncraftableItem => "UncraftableItem".to_string(),
            DroneActionError::MissingSlot => "MissingSlot".to_string(),
        }
    }
}

#[derive(Clone)]
pub enum DroneAction {
    PrimAction(DronePrimAction),
    AdvancedAction(DroneAdvancedAction),
    GetterAction(DroneGetterAction)
}

impl DroneAction {

    pub fn wrap_into_action(self) -> ActionType {
        ActionType::DroneAction(self)
    }

    pub fn execute(&self, drone: &mut Drone, world: &World, event_manager: &mut EventManager) -> Var {
        match self {
            DroneAction::PrimAction(drone_prim_action) => {
                return drone_prim_action.execute(drone, world, event_manager);
            },
            DroneAction::AdvancedAction(advanced_drone_action) => {
                return advanced_drone_action.execute(drone, world);
            },
            DroneAction::GetterAction(drone_getter_action) => {
                drone_getter_action.execute(drone)
            },
        }
    }

    pub fn get_all_actions() -> Vec<DroneAction> {
        let mut all_actions = Vec::new();
    
        all_actions.append(&mut DronePrimAction::get_all_actions());
        all_actions.append(&mut DroneAdvancedAction::get_all_actions());
        all_actions.append(&mut DroneGetterAction::get_all_actions());

        return all_actions;
    }

    //=====================================
    // Identity
    //=====================================

    pub fn get_name(&self) -> String {
        match self {
            DroneAction::PrimAction(drone_prim_action) => drone_prim_action.get_name(),
            DroneAction::AdvancedAction(advanced_drone_action) => advanced_drone_action.get_name(),
            DroneAction::GetterAction(drone_getter_action) => drone_getter_action.get_name(),
        }
    }

    pub fn get_texture(&self) -> Texture {
        match self {
            DroneAction::PrimAction(drone_prim_action) => {
                drone_prim_action.get_texture()
            },
            DroneAction::AdvancedAction(drone_advanced_action) => {
                drone_advanced_action.get_texture()
            },
            DroneAction::GetterAction(drone_getter_action) => {
                drone_getter_action.get_texture()
            },
        }
    }

    //=====================================
    // Function Managment
    //=====================================

    pub fn get_param_var_types(&self) -> Vec<VarKind> {
        match self {
            DroneAction::PrimAction(drone_prim_action) => {
                drone_prim_action.get_param_var_types()
            },
            DroneAction::AdvancedAction(advanced_drone_action) => {
                advanced_drone_action.get_param_var_kinds()
            },
            DroneAction::GetterAction(drone_getter_action) => {
                Vec::new()
            },
        }
    }

    pub fn get_return_var(&self) -> Option<Var> {
        match self {
            DroneAction::GetterAction(getter_action) => {
                Some(getter_action.get_return_var())
            }
            _ => {
                None
            }
        }
    }

    pub fn set_params_from_vars(&mut self, params: &Vec<Var>) {
        match self {
            DroneAction::PrimAction(drone_prim_action) => {
                // drone_prim_action.set_params_from_vars(params);
            },
            DroneAction::AdvancedAction(advanced_drone_action) => {
                
                advanced_drone_action.set_params_from_vars(params);
            },
            DroneAction::GetterAction(getter_action) => {

            }
        }
    }
}
