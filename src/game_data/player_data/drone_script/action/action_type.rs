use crate::game_data::player_data::{drone_script::var::{var::Var, var_type::{VarKind, VarType}}, drones::drone_actions::drone_actions::DroneAction};


#[derive(Clone)]
pub enum ActionType {
    DroneAction(DroneAction),
}


impl ActionType {
    pub fn get_param_var_kinds(&self) -> Vec<VarKind> {
        match self {
            ActionType::DroneAction(drone_action) => {
                drone_action.get_param_var_types()
            },
        }
    }

    pub fn get_return_var(&self) -> Option<Var> {
        match self {
            ActionType::DroneAction(drone_action) => {
                drone_action.get_return_var()
            },
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            ActionType::DroneAction(drone_action) => {
                drone_action.get_name()
            },
        }
    }
}