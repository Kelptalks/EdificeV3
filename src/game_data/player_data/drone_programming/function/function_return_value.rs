use crate::game_data::player_data::{drone_programming::script_element::{self, ScriptElement}, drones::drone_actions::drone_actions::{DroneAction, DroneActionError}};


#[derive(Clone)]
pub enum ErrorCode {
    DroneActionError(DroneActionError),
    
}

impl ErrorCode {
    pub fn to_string(&self) -> String {
        match self {
            ErrorCode::DroneActionError(drone_action_error) => drone_action_error.to_string(),
        }
    }
}

#[derive(Clone)]
pub enum FunctionReturnValue {
    // Satus
    Fail(ErrorCode),
    Ok(),
    
    // Actions
    ScriptElement(ScriptElement),
    Action(DroneAction),
    

    // Prim
    Bool(bool),
    
}

impl FunctionReturnValue {
    pub fn to_string(&self) -> String {
        match self {
            FunctionReturnValue::Fail(error_code) => error_code.to_string(),
            FunctionReturnValue::Ok() => "Ok".to_string(),
            
            FunctionReturnValue::ScriptElement(script_element) => {
                script_element.get_name()
            }
            FunctionReturnValue::Action(action) => {
                action.get_name()
            }

            FunctionReturnValue::Bool(bool) => bool.to_string(),

            
        }
    }
}