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
