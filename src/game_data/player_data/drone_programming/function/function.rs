use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::{drone_programming::var::var_type::Var, drones::drone_actions::drone_actions::DroneAction}, screen::{camera_controls, widget::prelude::VarSlot}};

pub struct Function {
    params: Vec<Rc<RefCell<Var>>>,
    
    drone_action: DroneAction,

}

impl Function {
    pub fn new_from_drone_action(action: DroneAction) -> Function {
        let params = action.create_param_vars();
        Function {
            params: params,
            drone_action: action
        }
    }


    pub fn get_params(&self) -> &Vec<Rc<RefCell<Var>>> {
        return &self.params;
    }

    pub fn into_drone_action(&self) -> DroneAction {
        let mut constructed_action = self.drone_action.clone();
        constructed_action.set_params_from_vars(&self.params);
        return constructed_action;
    }


}