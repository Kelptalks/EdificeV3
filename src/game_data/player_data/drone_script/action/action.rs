use crate::game_data::player_data::drone_script::{action::action_type::ActionType, var::var::{Var, VarRef}};


#[derive(Clone)]
pub struct Action {
    action_type: ActionType,   
    
    params: Vec<VarRef>,
    
}

impl Action {
    pub fn new(action_type: ActionType) -> Action {
        // Create var refs with types
        
        let var_types = action_type.get_param_var_types();
        let mut var_refs = Vec::new();
        for var_type in var_types {
            var_refs.push(VarRef::new_blank_with_kind(var_type.to_kind()));
        }


        Action {
            action_type: action_type,
            params: var_refs,
        }
    }
    
    pub fn get_name(&self) -> String {
        "NO NAME FOR ACTION".to_string()
    }
}