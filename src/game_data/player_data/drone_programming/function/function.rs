use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::{drone_programming::{function::function_return_value::FunctionReturnValue, var::var_type::Var}, drones::drone_actions::{drone_actions::DroneAction, prim_actions::{drone_prim_actions::DronePrimAction, drone_world_actions::DroneWorldAction}}}, screen::{camera_controls, widget::prelude::VarSlot}};

#[derive(Clone)]
pub struct Function {
    params: Vec<Rc<RefCell<Var>>>,
    
    action: DroneAction,

    return_value: Option<FunctionReturnValue>,

}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        eprintln!("Part Equal not implemented for Function");
        todo!()
    }
}

impl Function {
    pub fn new_blank() -> Function {
        Function {
            params: Vec::new(),
            
            action: DroneWorldAction::MoveDrone([0, 0, 0]).wrap_into_action(),

            return_value: None,
        }
    }

    pub fn new_from_drone_action(action: DroneAction) -> Function {
        let params = action.create_param_vars();
        Function {
            params: params,
            
            action,

            return_value: None,
        }
    }


    pub fn get_params(&self) -> &Vec<Rc<RefCell<Var>>> {
        return &self.params;
    }

    pub fn into_drone_action(&self) -> DroneAction {
        let mut constructed_action = self.action.clone();
        constructed_action.set_params_from_vars(&self.params);
        return constructed_action;
    }

    pub fn get_return_value(&self) -> &Option<FunctionReturnValue> {
        &self.return_value
    }


    pub fn get_name(&self) -> String {

        return self.action.get_name();
    }


}