use std::{cell::RefCell, rc::Rc};

use crate::game_data::{
    player_data::{
        drone_programming::{function::function::Function, var::var_type::Var}, 
        drones::drone_actions::drone_actions::DroneAction}, 
    screen::widget::{drone_programming::{function_slot::FunctionSlot}, panel::panel::Panel, prelude::VarSlot, widget::WidgetType}};

#[derive(Clone)]
pub enum ScriptElement {
    Function(Rc<RefCell<Function>>),    

    
    Action(DroneAction),
}

impl PartialEq for ScriptElement {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}


impl ScriptElement {
    pub fn get_name(&self) -> String {
        match self {
            ScriptElement::Function(function) => function.borrow().get_name(),
            
            ScriptElement::Action(drone_action) => drone_action.get_name(),
        }
    }

    pub fn construct_widget(&self) -> WidgetType {
        match self {
            ScriptElement::Function(function) => {
                return FunctionSlot::new_with_function_ref(function).wrap_into_widget()
            },
            ScriptElement::Action(drone_action) => {
                let mut panel = Panel::new_blank();
                panel.add_text_display(drone_action.get_name());
                return panel.wrap_into_widget();
            },
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