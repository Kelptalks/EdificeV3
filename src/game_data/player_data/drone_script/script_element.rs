use std::{cell::RefCell, rc::Rc};

use crate::game_data::{
    player_data::{
        drone_script::{action::action::Action, function::{function::Function, function_call::{self, FunctionCall}}, var::{var::{Var, VarRef}, var_type::VarType}}, 
        drones::drone_actions::drone_actions::DroneAction}, 
    screen::widget::{drone_programming::function_slot::FunctionSlot, panel::panel::Panel, prelude::VarSlot, widget::WidgetType}};

#[derive(Clone)]
pub enum ScriptElement {    
    // Vars
    Var(Var),
    VarRef(VarRef),
    
    // Game actions
    Action(Action),

    // Functions
    FunctionCall(FunctionCall),
    Function(Function),       
    


}

impl PartialEq for ScriptElement {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}


impl ScriptElement {
    pub fn get_name(&self) -> String {
        match self {
            ScriptElement::Var(var) => var.get_name(),
            ScriptElement::VarRef(var_ref) => var_ref.get_name(),
            
            ScriptElement::Function(function) => function.get_name(),
            ScriptElement::FunctionCall(function_call) => function_call.get_name(),

            ScriptElement::Action(action) => action.get_name(),
        }
    }

}


/*
ScriptElement::Params(paramas) => {
                let mut string = "Params (".to_string();
                for param in paramas {
                    string = format!("{} {}", string, param.borrow().get_name());
                }


                return string;
            }
*/