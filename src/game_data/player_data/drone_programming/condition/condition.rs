
use crate::game_data::player_data::drone_programming::{function::function::Function, script_element::ScriptElement};


#[derive(Clone, PartialEq)]
pub struct Condition {
    condition_function: Function,

    script_elements: Vec<ScriptElement>,
}

impl Condition {

    pub fn new() -> Condition {
        Condition {
            condition_function: Function::new_blank(),
            script_elements: Vec::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.condition_function.get_name()
    }


}