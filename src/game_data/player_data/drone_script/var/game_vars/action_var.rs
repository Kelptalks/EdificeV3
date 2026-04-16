use crate::game_data::{player_data::{drone_script::{script_element::{self, ScriptElement}, var::{game_vars::game_var_type::GameVarType, var_type::VarType}}, drones::drone_actions::drone_actions::{DroneAction, DroneActionError}}, texture_manager::texture::Texture};


#[derive(Clone)]
pub enum ErrorCode {
    DroneActionError(DroneActionError),
    
}

impl ErrorCode {
    pub fn wrap_into_var_type(self) -> VarType {
        ActionVarType::Status(self).wrap_into_var_type()
    }
    
    pub fn to_string(&self) -> String {
        match self {
            ErrorCode::DroneActionError(drone_action_error) => drone_action_error.get_name(),
        }
    }
}

#[derive(Clone)]
pub enum ActionVarType {
    // Satus
    Status(ErrorCode),

    // Actions
    Action(DroneAction),
}

impl ActionVarType {
    pub fn wrap_into_var_type(self) -> VarType {
        GameVarType::Action(self).wrap_into_var_type()
    }
    
    pub fn get_texture(&self) -> Texture {
        match self {
            ActionVarType::Status(error_code) => todo!(),
            ActionVarType::Action(drone_action) => todo!(),
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            ActionVarType::Status(error_code) => error_code.to_string(),
            ActionVarType::Action(action) => {
                action.get_name()
            }
        }
    }
}


pub enum ActionVarKind {
    Status(),
    Action(),
}