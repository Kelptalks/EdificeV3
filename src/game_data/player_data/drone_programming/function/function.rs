use std::{cell::{Ref, RefCell}, collections::HashMap, rc::Rc};

use crate::game_data::{player_data::{drone_programming::{compiled_script_element::CompiledScripElement, function::function_return_value::FunctionReturnValue, script_element::{self, ScriptElement}, var::var_type::Var}, drones::drone_actions::{drone_actions::DroneAction, prim_actions::{drone_prim_actions::DronePrimAction, drone_world_actions::DroneWorldAction}}}, screen::{camera_controls, widget::prelude::VarSlot}};

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
        
        // Params
        let params = action.create_param_vars();
        
        let mut script_elements = Vec::new();
        script_elements.push(ScriptElement::Action(action.clone()));

        // Return
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

            script_elements: script_elements,            

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

    pub fn get_return(&mut self) -> &mut Vec<(FunctionReturnValue, ScriptElement)> {
        &mut self.return_functions
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_length(&self) -> usize {
        let mut length = 0;
        
        for script_element in &self.script_elements {
            match script_element {
                ScriptElement::Function(ref_cell) => length += ref_cell.borrow().get_length(),
                ScriptElement::Action(drone_action) => length += 1,
            }
        }
        
        for (return_value, script_element) in &self.return_functions {
            match script_element {
                ScriptElement::Function(ref_cell) => length += ref_cell.borrow().get_length(),
                ScriptElement::Action(drone_action) => length += 1,
            }
        }

        length
    }    


    pub fn flatten(&mut self, indent: usize) -> Vec<(usize, ScriptElement)> {
        let mut flattened_elements = Vec::new();

        // LOOP THROUGH ALL ELEMENTS

        for var in &self.params {
            
        }


        for script_element in &mut self.script_elements {
            flattened_elements.push((indent, script_element.clone()));
            match script_element {
                ScriptElement::Function(ref_cell) => {
                    flattened_elements.append(&mut ref_cell.borrow_mut().flatten(indent + 1));
                },
                _ => {

                }
            }
        }
        
        for (return_value, script_element) in &mut self.return_functions {
            flattened_elements.push((indent, script_element.clone()));
            
            match script_element {
                ScriptElement::Function(ref_cell) => {
                    flattened_elements.append(&mut ref_cell.borrow_mut().flatten(indent + 1));
                },
                _ => {

                }
            }
        }

        // Calculate index skips based off function length    
        flattened_elements

    }

    pub fn compile(&mut self, indent: usize) -> Vec<(usize, CompiledScripElement)> {
        let mut compiled_elements = Vec::new();

        // LOOP THROUGH ALL ELEMENTS

        for var in &self.params {
            
        }


        for script_element in &mut self.script_elements {
            match script_element {
                ScriptElement::Function(ref_cell) => {
                    compiled_elements.append(&mut ref_cell.borrow_mut().compile(indent + 1));
                },
                ScriptElement::Action(action) => {
                    compiled_elements.push((indent, CompiledScripElement::DroneAction(action.clone())));
                }
                _ => {

                }
            }
        }
        
        let mut return_function = Vec::new();
        for (return_value, script_element) in &mut self.return_functions {
            match script_element {
                ScriptElement::Function(ref_cell) => {
                    let element = 
                        CompiledScripElement::ReturnValueToIndexMod(
                            return_value.clone(), 
                            ref_cell.borrow().get_length()
                        );
                    compiled_elements.push((indent, element));
                    
                    
                    return_function.append(&mut ref_cell.borrow_mut().compile(indent));
                },
                ScriptElement::Action(action) => {
                    compiled_elements.push((indent, CompiledScripElement::DroneAction(action.clone())));
                }
                _ => {
                    
                }
            }
        }

        compiled_elements.append(&mut return_function);

        // Calculate index skips based off function length
        


        
        compiled_elements


    }

}