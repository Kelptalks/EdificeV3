use crate::game_data::player_data::drone_programming::script_element::ScriptElement;



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

    pub fn get_name(&self) -> String {
        return self.name.to_string();
    }
}

