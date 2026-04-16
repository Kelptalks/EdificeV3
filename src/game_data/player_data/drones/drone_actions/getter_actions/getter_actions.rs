use crate::game_data::player_data::{drone_script::var::{prim_vars::prim_var_type::PrimitiveVarType, var::Var, var_type::VarType}, drones::{drone::Drone, drone_actions::drone_actions::{DroneAction, DroneActionError}}};

#[derive(Clone)]
pub enum DroneGetterAction {
    IsBusy,
}

impl DroneGetterAction {
    pub fn wrap_into_action(self) -> DroneAction {
        DroneAction::GetterAction(self)
    }
    
    pub fn execute(&self, drone: &mut Drone) -> Var {
        match self {
            DroneGetterAction::IsBusy => DroneActionError::Ok.wrap_into_var_type().create_var(),
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

    pub fn create_return_values(&self) -> Vec<VarType> {
        let mut return_values=  Vec::new();
        match self {
            DroneGetterAction::IsBusy => {
                return_values.push(PrimitiveVarType::Bool(true).wrap_into_var_type());
                return_values.push(PrimitiveVarType::Bool(false).wrap_into_var_type());
            }
        }
        return_values
    }

    

    
}