use crate::game_data::player_data::drone_script::script_element::ScriptElement;


#[derive(Clone)]
pub struct ScriptElementBody {
    pub elements: Vec<ScriptElement>

}

impl ScriptElementBody {
    pub fn new() -> ScriptElementBody {
        ScriptElementBody {
            elements: Vec::new()
        }
    }

    pub fn incert_element(&mut self, index: usize, element: ScriptElement) {
        self.elements.insert(index, element);
    }
    
    
    pub fn remove_element(&mut self, index: usize) {
        if index < self.elements.len() {
            self.elements.remove(index);
        }
        else {
            eprintln!("script element index out of range of function body")
        }
    }

    pub fn get_mut_element(&mut self, index: usize) -> Option<&mut ScriptElement> {
        return self.elements.get_mut(index);
    }

}