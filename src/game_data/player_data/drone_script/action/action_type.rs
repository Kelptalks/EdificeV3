use crate::game_data::player_data::{drone_script::var::{var::Var, var_type::VarType}, drones::drone_actions::drone_actions::DroneAction};


#[derive(Clone)]
pub enum ActionType {
    DroneAction(DroneAction),
}


impl ActionType {
    pub fn get_param_var_types(&self) -> Vec<VarType> {
        match self {
            ActionType::DroneAction(drone_action) => todo!(),
        }
    }
}