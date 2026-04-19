use std::{cell::RefCell, rc::Rc};

use crate::game_data::{
    player_data::{
        drone_script::{action::action::Action, control_flow::control_flow::ControlFlow, function::{function::Function, function_call::{self, FunctionCall}}, var::{programming_vars::programming_var::ProgrammingVar, var::{Var, VarRef}, var_type::VarType}}, 
        drones::drone_actions::drone_actions::DroneAction}, 
    screen::widget::{drone_programming::{action_slot::ActionSlot, function_slot::FunctionSlot}, panel::panel::Panel, prelude::VarSlot, widget::WidgetType}};

#[derive(Clone)]
pub enum ScriptElement {    
    // Vars
    Var(Var),
    VarRef(VarRef),
    
    // Control Flow
    ControlFlow(ControlFlow),

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
            
            ScriptElement::ControlFlow(control_flow) => control_flow.get_name(), 

            ScriptElement::Function(function) => function.get_name(),
            ScriptElement::FunctionCall(function_call) => function_call.get_name(),

            ScriptElement::Action(action) => action.get_name(),
        }
    }

    pub fn wrap_into_var_type(self) -> VarType {
        ProgrammingVar::ScriptingElement(self).wrap_into_var()
    }

    pub fn create_widget(&self) -> WidgetType {

        match self {
            ScriptElement::Var(var) => {
                return VarSlot::new_with_var(var.clone()).wrap_into_widget()
            },
            ScriptElement::VarRef(var_ref) => {
                return VarSlot::new_with_var_ref(var_ref.clone()).wrap_into_widget()
            },

            ScriptElement::ControlFlow(contorl_flow) => {
                let mut panel = Panel::new_blank();

                panel.add_text_display(contorl_flow.get_name());

                return panel.wrap_into_widget()
            }

            ScriptElement::Action(action) => {
                return ActionSlot::new(action).wrap_into_widget()
            },
            ScriptElement::FunctionCall(function_call) => {
                
            },
            ScriptElement::Function(function) => {
                return FunctionSlot::new_with_function(function).wrap_into_widget()
            },
        }

        return Panel::new_blank().wrap_into_widget()
    }

}
