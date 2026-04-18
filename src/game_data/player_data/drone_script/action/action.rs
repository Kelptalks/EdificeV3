use crate::game_data::player_data::drone_script::{action::action_type::ActionType, script_element::ScriptElement, var::{prim_vars::prim_var_type::PrimitiveVarType, var::{Var, VarRef}, var_type::VarType}};


#[derive(Clone)]
pub struct Action {
    action_type: ActionType,   
    
    params: Vec<VarRef>,
    

    return_var: Option<Var>,

}

impl Action {
    pub fn new(action_type: ActionType) -> Action {
        
        // Create var refs with types from action types params
        let var_types = action_type.get_param_var_types();
        let mut var_refs = Vec::new();
        for var_type in var_types {
            var_refs.push(VarRef::new_blank_with_kind(var_type.to_kind()));
        }




        Action {
            action_type: action_type,
            params: var_refs,

            return_var: Some(Var::new_with_var_type(VarType::Prim(PrimitiveVarType::Bool(false)))),
        }
    }
    
    pub fn wrap_into_script_element(self) -> ScriptElement {
        ScriptElement::Action(self)
    }

    pub fn get_params_var_refs(&self) -> &Vec<VarRef> {
        &self.params
    }

    pub fn get_name(&self) -> String {
        self.action_type.get_name()
    }

    pub fn get_return_var(&self) -> &Option<Var> {
        &self.return_var
    }
}