use crate::game_data::player_data::drones::drone_actions::drone_actions::DroneActionError;


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
    Ok(),
    Bool(bool),
    Fail(ErrorCode)
}

impl FunctionReturnValue {
    pub fn to_string(&self) -> String {
        match self {
            FunctionReturnValue::Ok() => "Ok".to_string(),
            FunctionReturnValue::Bool(bool) => bool.to_string(),
            FunctionReturnValue::Fail(error_code) => error_code.to_string(),
        }
    }
}