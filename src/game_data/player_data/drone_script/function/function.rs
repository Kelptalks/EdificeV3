use std::{collections::VecDeque, usize};

use crate::game_data::player_data::drone_script::{
            element_body::ScriptElementBody, script_element::ScriptElement, var::var::{Var}
        };

#[derive(Clone)]
pub struct Function {
    name: String,

    paused: bool,
    execution_index_key: VecDeque<usize>,

    params: Vec<Var>,
    body: ScriptElementBody,

    return_value: Var,
}

impl PartialEq for Function {
    fn eq(&self, _other: &Self) -> bool {
        eprintln!("Part Equal not implemented for Function");
        todo!()
    }
}

impl Function {
    pub fn new_blank() -> Function {
        Function {
            name: "Blank Function".to_string(),

            // Execution
            paused: true,
            execution_index_key: VecDeque::new(),

            // Values
            params: Vec::new(),
            body: ScriptElementBody::new(),
            return_value: Var::new_blank(),
        }
    }

    pub fn new_drone_function() -> Function {
        let mut function = Function::new_blank();
        function.set_name("Drone Function");

        function
    }

    pub fn to_script_element(&self) -> ScriptElement {
        ScriptElement::Function(self.clone())
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }

    //=====================================
    // Execution
    //=====================================

    pub fn is_paused(&self) -> bool {
        return self.paused;
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused
    }

    pub fn step_function(&mut self) {
        if self.paused {
            return;
        }
        let cloned_key = self.execution_index_key.clone();
        if let Some(current_key) = self.execution_index_key.front_mut() {
            if let Some(current_element) = self.body.get_mut_element_with_key(&mut cloned_key.clone()) {

                if current_element.has_body() {
                    self.execution_index_key.push_front(0);
                }
                else {
                    *current_key += 1;
                }
            }
            else {
                // Function Finished
                self.execution_index_key.pop_front();
                if let Some(new_front) = self.execution_index_key.front_mut() {
                    *new_front += 1;
                }
            }
        }
        else {
            // Start Function
            self.execution_index_key.push_front(0);
        }        
        
    } 

    pub fn get_current_execution_element(&mut self) -> Option<&mut ScriptElement> {
        let mut cloned_key = self.execution_index_key.clone();
        return self.body.get_mut_element_with_key(&mut cloned_key);
    }

    pub fn get_execution_index_key(&self) -> &VecDeque<usize> {
        &self.execution_index_key
    }

    pub fn set_execution_index_key(&mut self, key: VecDeque<usize>) {
        self.execution_index_key = key;
    }

    //=====================================
    // Values
    //=====================================
    
    // Params
    pub fn get_params(&self) -> &Vec<Var> {
        &self.params
    }

    // Body
    pub fn get_mut_body(&mut self) -> &mut ScriptElementBody {
        return &mut self.body
    }
    pub fn get_body(&self) -> &ScriptElementBody {
        return &self.body
    }

    // Return
    pub fn get_return_var(&mut self) -> &Var {
        &self.return_value
    }


}


#[cfg(test)]
mod tests {
    use crate::game_data::player_data::{drone_script::{action::{action::Action, cd}, control_flow::control_flow::ControlFlow, function::function::Function, script_element::{self, ScriptElement}}, drones::drone_actions::{advanced_actions::advanced_drone_actions::DroneAdvancedAction, drone_actions::DroneAction, prim_actions::{drone_prim_actions::DronePrimAction, drone_world_actions::DroneWorldAction}}};

    

    #[test]
    fn test_function_stepping() {
        let mut function = Function::new_blank();

        let action = DroneAdvancedAction::PathToLocation(None).wrap_into_action();
        let path = ScriptElement::Action(Action::new(action));
        
        let action = DronePrimAction::DroneWorldAction(DroneWorldAction::MineBlock([0, 0, 0])).wrap_into_action().wrap_into_action();
        let mine = ScriptElement::Action(Action::new(action));



        function.body.push_element(path.clone());

        let mut control_flow = ControlFlow::new_blank();
        for i in 0..4 {
            control_flow.get_mut_body().push_element(mine.clone());
        }

        let control_flow_script_element = ScriptElement::ControlFlow(control_flow);
        function.body.push_element(control_flow_script_element);


        function.body.push_element(path.clone());
        function.body.push_element(path.clone());
        
        for i in 0..10 {
            function.step_function();
        }

        println!("{:?}", function.get_execution_index_key());

    }
}