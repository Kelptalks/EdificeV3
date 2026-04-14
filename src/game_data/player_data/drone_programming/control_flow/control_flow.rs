
use std::{cell::RefCell, rc::Rc};

use crate::game_data::player_data::drone_programming::{function::{self, function::Function}, script_element::ScriptElement};


#[derive(Clone, PartialEq)]
pub struct ControlFlow {
    condition_function: Option<ScriptElement>,
    script_elements: Vec<ScriptElement>,
}

impl ControlFlow {

    pub fn new() -> ControlFlow {
        ControlFlow {
            condition_function: None,
            script_elements: Vec::new(),
        }
    }

    pub fn get_function(&self) -> &Option<ScriptElement> {
        return &self.condition_function
    }

    pub fn set_function(&mut self, script_element: ScriptElement) {
        if let ScriptElement::Function(_) = script_element {
            self.condition_function = Some(script_element); 
        }
    }

    pub fn into_script_element(self) -> ScriptElement {
        ScriptElement::ControlFlow(Rc::new(RefCell::new(self)))
    }

    pub fn get_name(&self) -> String {
        return "If".to_string()
    }


}