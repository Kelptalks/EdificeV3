use crate::game_data::player_data::{drone_programming::function::function_return_value::FunctionReturnValue, drones::{drone::Drone, drone_actions::drone_actions::DroneAction}};

#[derive(Clone)]
pub enum DroneGetterAction {
    IsBusy,
}

impl DroneGetterAction {
    pub fn wrap_into_action(self) -> DroneAction {
        DroneAction::GetterAction(self)
    }
    
    pub fn execute(&self, drone: &mut Drone) -> FunctionReturnValue {
        match self {
            DroneGetterAction::IsBusy => FunctionReturnValue::Bool(drone.is_busy()),
        }
    }

    pub fn get_all_actions() -> Vec<DroneAction> {
        let mut all_actions = Vec::new();

        all_actions.push(DroneGetterAction::IsBusy.wrap_into_action());

        all_actions
    }

    //=====================================
    // Identity
    //=====================================

    pub fn get_name(&self) -> String {
        match self {
            DroneGetterAction::IsBusy => "IsBusy".to_string(),
        }
    }

    //=====================================
    // Function Managment
    //=====================================

    pub fn create_return_values(&self) -> Vec<FunctionReturnValue> {
        let mut return_values=  Vec::new();
        match self {
            DroneGetterAction::IsBusy => {
                return_values.push(FunctionReturnValue::Bool(true));
                return_values.push(FunctionReturnValue::Bool(false));
            }
        }
        return_values
    }

    

    
}