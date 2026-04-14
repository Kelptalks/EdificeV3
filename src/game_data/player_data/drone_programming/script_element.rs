use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::{drone_programming::{control_flow::control_flow::ControlFlow, function::function::Function}, drones::drone_actions::drone_actions::DroneAction}, screen::widget::{drone_programming::scripting_elements::scripting_element_widgets::{control_flow_widget::ControlFlowWidget, function_widget::FunctionWidget}, widget::WidgetType}};

#[derive(Clone)]
pub enum ScriptElement {
    Function(Rc<RefCell<Function>>),
    ControlFlow(Rc<RefCell<ControlFlow>>),
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
            ScriptElement::ControlFlow(condition) => condition.borrow().get_name(),
            ScriptElement::Action(drone_action) => drone_action.get_name(),
        }
    }

    pub fn construct_widget(&self) -> WidgetType {
        match self {
            ScriptElement::Function(function) => {
                return FunctionWidget::new(function).wrap_into_widget()
            },
            ScriptElement::ControlFlow(control_flow) => {
                return ControlFlowWidget::new(control_flow).wrap_into_widget()
            },
            ScriptElement::Action(drone_action) => {
                todo!()
            },
        }
    }
}