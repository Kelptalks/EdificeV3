use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::game_data::{player_data::{drone_programming::{function::function_return_value::FunctionReturnValue, script_element::{self, ScriptElement}, var::var_type::Var}, drones::drone_actions::{drone_actions::DroneAction, prim_actions::{drone_prim_actions::DronePrimAction, drone_world_actions::DroneWorldAction}}}, screen::{camera_controls, widget::prelude::VarSlot}};

#[derive(Clone)]
pub struct Function {
    name: String,
    
    params: Vec<Rc<RefCell<Var>>>,

    script_elements: Vec<ScriptElement>,

    return_functions: Vec<(FunctionReturnValue, ScriptElement)>,
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
            name: "Blank Function".to_string(),

            params: Vec::new(),
            
            script_elements: Vec::new(),

            return_functions: Vec::new(),
        }
    }

    pub fn new_from_drone_action(action: DroneAction) -> Function {
        let name = action.get_name();
        
        let params = action.create_param_vars();
        

        let mut return_functions: Vec<(FunctionReturnValue, ScriptElement)> = Vec::new();

        let return_values = action.create_return_values();
        
        for return_value in return_values {
            let mut function = Function::new_blank();
            function.name = return_value.to_string();
            return_functions.push((return_value, function.to_script_element()))
        }
        

        Function {
            name: name,
            params: params,  

            script_elements: Vec::new(),            

            return_functions: return_functions,
        }
    }


    pub fn to_script_element(self) -> ScriptElement {
        ScriptElement::Function(Rc::new(RefCell::new(self)))
    }


    pub fn get_params(&self) -> &Vec<Rc<RefCell<Var>>> {
        &self.params
    }

    pub fn add_script_element(&mut self, index: usize, script_element: ScriptElement) {
        self.script_elements.push(script_element);
    }
    
    pub fn get_script_elements(&self) -> &Vec<ScriptElement> {
        return &self.script_elements
    }

    pub fn get_return_functions(&self) -> Vec<ScriptElement> {
        self.return_functions
            .iter()
            .map(|(_, script_elem)| script_elem.clone())
            .collect()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

}