use crate::game_data::player_data::drone_programming::{condition::condition::Condition, function::function::Function};

#[derive(Clone)]
pub enum ScriptElement {
    Function(Function),
    Condition(Condition),
}

impl PartialEq for ScriptElement {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}


impl ScriptElement {
    pub fn get_name(&self) -> String {
        match self {
            ScriptElement::Function(function) => function.get_name(),
            ScriptElement::Condition(condition) => condition.get_name(),
        }
    }
}