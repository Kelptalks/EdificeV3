use std::{cell::RefCell, rc::Rc};

use crate::game_data::{
    player_data::{
        drone_script::{action::action::Action, control_flow::control_flow::ControlFlow, element_body::ScriptElementBody, function::{function::Function, function_call::{self, FunctionCall}}, var::{programming_vars::programming_var::ProgrammingVar, var::{Var, VarRef}, var_type::VarType}}, 
        drones::drone_actions::drone_actions::DroneAction}, 
    screen::widget::{drone_programming::{action_slot::ActionSlot, control_flow_slot::ControlFlowSlot, function_slot::FunctionSlot}, panel::panel::Panel, prelude::VarSlot, widget::WidgetType}, texture_manager::texture::Texture, types::UITextures};

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
        todo!()
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

    pub fn create_widget(&self, line: usize) -> WidgetType {

        match self {
            ScriptElement::Var(var) => {
                return VarSlot::new_with_var(var.clone()).wrap_into_widget()
            },
            ScriptElement::VarRef(var_ref) => {
                return VarSlot::new_with_var_ref(var_ref.clone()).wrap_into_widget()
            },

            ScriptElement::ControlFlow(control_flow) => {
                return ControlFlowSlot::new(control_flow, line).wrap_into_widget()
            }

            ScriptElement::Action(action) => {
                return ActionSlot::new(action, line).wrap_into_widget()
            },
            ScriptElement::FunctionCall(function_call) => {
                
            },
            ScriptElement::Function(function) => {
                eprintln!("Cannot create function slot from script element because fuction slot uses a ref");
            },
        }

        return Panel::new_blank().wrap_into_widget()
    }

    pub fn get_texture(&self) -> Texture {
        match self {
            ScriptElement::Var(var) => {
                var.get_texture()
            },
            ScriptElement::VarRef(var_ref) => {
                var_ref.get_texture()
            },
            ScriptElement::ControlFlow(control_flow) => {
                UITextures::ControlFlowIcon.wrap_into_texture()
            },
            ScriptElement::Action(action) => {
                action.get_texture()
            },
            ScriptElement::FunctionCall(function_call) => todo!(),
            ScriptElement::Function(function) => todo!(),
        }
    }

    pub fn has_body(&self) -> bool {
        match self {
            ScriptElement::Function(_) => true,
            ScriptElement::ControlFlow(_) => true,
            _ => {
                false
            }
        }
    }

    pub fn get_mut_body(&mut self) -> Option<&mut ScriptElementBody> {

        match self {
            ScriptElement::ControlFlow(control_flow) => {
                println!("Got control flow body");
                return Some(control_flow.get_mut_body())
            }
            _ => {
                return None
            }
        }
    }

}
