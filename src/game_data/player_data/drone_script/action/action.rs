use crate::game_data::{player_data::{drone_script::{action::action_type::ActionType, script_element::ScriptElement, var::{prim_vars::prim_var_type::PrimitiveVarType, var::{Var, VarRef}, var_type::VarType}}, drones::drone_actions::drone_actions::DroneAction}, screen::ui_elements::text_bar::TextBar, texture_manager::texture::Texture};


#[derive(Clone)]
pub struct Action {
    action_type: ActionType,   
    
    params: Vec<Var>,
    

    return_var: Option<Var>,

}

impl Action {
    pub fn new(action_type: ActionType) -> Action {
        
        // Create varwith types from action types params
        let var_kinds = action_type.get_param_var_kinds();
        let mut vars = Vec::new();
        for var_kind in var_kinds {
            let var = Var::new_blank_with_kind(var_kind);
            vars.push(Var::new_blank_with_kind(var_kind));
        }

        

        let return_var = action_type.get_return_var();


        Action {
            action_type: action_type,
            params: vars,
            return_var,
        }
    }

    pub fn as_drone_action(&mut self) -> Option<DroneAction> {
        if let ActionType::DroneAction(drone_action) = &self.action_type{
            let mut compiled_action = drone_action.clone();
            compiled_action.set_params_from_vars(&self.params);
            Some(compiled_action)
        }
        else {
            None
        }
        
        

    }
    
    pub fn wrap_into_script_element(self) -> ScriptElement {
        ScriptElement::Action(self)
    }

    pub fn get_params_vars(&self) -> &Vec<Var> {
        &self.params
    }

    pub fn get_name(&self) -> String {
        self.action_type.get_name()
    }

    pub fn get_return_var(&self) -> &Option<Var> {
        &self.return_var
    }

    pub fn get_texture(&self) -> Texture {
        self.action_type.get_texture()
    }   
}