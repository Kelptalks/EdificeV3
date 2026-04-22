use std::collections::VecDeque;

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

    //=====================================
    // Basic Minipulation
    //=====================================

    pub fn incert_element(&mut self, index: usize, element: ScriptElement) {
        if index <= self.elements.len() {
            self.elements.insert(index, element);
        }

    }
    
    pub fn remove_element(&mut self, index: usize) {
        if index < self.elements.len() {
            self.elements.remove(index);
        }
        else {
            eprintln!("script element index out of range of function body")
        }
    }

    pub fn push_element(&mut self, element: ScriptElement) {
        self.elements.push(element);
    }


    pub fn get_mut_element(&mut self, index: usize) -> Option<&mut ScriptElement> {
        return self.elements.get_mut(index);
    }

    pub fn get_mut_element_with_key(&mut self, keys: &mut VecDeque<usize>) -> Option<&mut ScriptElement> {
        if let Some(key) = keys.pop_back() {
            
            if keys.is_empty() {
                return self.get_mut_element(key)
            }
            else {
                if let Some(element) = self.get_mut_element(key) {
                    if let Some(element_body) = element.get_mut_body() {
                        return element_body.get_mut_element_with_key(keys)
                    }
                }
            }
        }
        None
        
    }

    pub fn insert_element_with_key(&mut self, keys: &mut VecDeque<usize>, element_to_add: ScriptElement) {
        if let Some(key) = keys.pop_back() {
            
            if keys.is_empty() {
                self.incert_element(key, element_to_add);
            }
            else {
                if let Some(element) = self.get_mut_element(key) {

                    if let Some(element_body) = element.get_mut_body() {
                        element_body.insert_element_with_key(keys, element_to_add);
                    }
                }
            }
        }
        
    }

}