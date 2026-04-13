use crate::game_data::player_data::drone_programming::{function::function::Function, script_element::ScriptElement};



#[derive(Clone, PartialEq)]
pub struct Script {
    name: String,
    
    elements: Vec<ScriptElement>,
}



impl Script {
    
    pub fn new() -> Script {
        Script {
            name: "Unnamed".to_string(),
            elements: Vec::new(),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_name(&self) -> String {
        return self.name.to_string();
    }

    pub fn get_elements(&self) -> &Vec<ScriptElement> {
        &self.elements
    }

    pub fn add_function(&mut self, index: usize, function: Function) {
        self.elements.insert(index, ScriptElement::Function(function));
    }
}

