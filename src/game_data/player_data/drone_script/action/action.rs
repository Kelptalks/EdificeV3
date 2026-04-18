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
        let var_kinds = action_type.get_param_var_kinds();
        let mut var_refs = Vec::new();
        for var_kind in var_kinds {
            var_refs.push(VarRef::new_blank_with_kind(var_kind));
        }

        

        let return_var = action_type.get_return_var();


        Action {
            action_type: action_type,
            params: var_refs,

            return_var,
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