use crate::game_data::player_data::{drone_programming::{function::function_return_value::FunctionReturnValue, var::var_type::Var}, drones::drone_actions::drone_actions::DroneAction};

pub enum CompiledScripElement {
    DroneAction(DroneAction), // Params, return value index offsets
    
    ReturnValueToIndexMod(FunctionReturnValue, usize)

}

impl CompiledScripElement {
    pub fn get_name(&self) -> String {
        match self {
            CompiledScripElement::DroneAction(drone_action) => {
                drone_action.get_name()
            },
            CompiledScripElement::ReturnValueToIndexMod(function_return_value, index) => {
                format!("{} ModIndex {}", function_return_value.to_string(), index)
            },
        }
    }
}