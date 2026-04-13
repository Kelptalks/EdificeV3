use crate::game_data::player_data::{drone_programming::{function::function::Function, script_element::ScriptElement}, drones::drone_actions::drone_actions::DroneAction};



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


    pub fn tik(&mut self) -> Vec<DroneAction> {
        let mut drone_actions = Vec::new();

        for element in &mut self.elements {
            if let ScriptElement::Function(function) = element {
                drone_actions.push(function.into_drone_action());
            }
        }

        println!("tiking");

        return drone_actions;
    }
}

