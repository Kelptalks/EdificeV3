use crate::game_data::player_data::drone_script::var::var::{Var, VarRef};


#[derive(Clone)]
pub struct Action {
    params: Vec<VarRef>,
    
}

impl Action {
    pub fn get_name(&self) -> String {
        "NO NAME FOR ACTION".to_string()
    }
}