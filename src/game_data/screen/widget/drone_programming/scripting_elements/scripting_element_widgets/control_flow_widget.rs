use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_programming::control_flow::control_flow::ControlFlow, screen::widget::{drone_programming::scripting_elements::scripting_element_widget_manager::ScriptingElementWidget, panel::panel::Panel, widget::WidgetType}};

pub struct ControlFlowWidget {
    panel: Panel,


    
}



impl ControlFlowWidget {
    pub fn new(control_flow: &Rc<RefCell<ControlFlow>>) -> ControlFlowWidget {
        let borrowed_control_flow = control_flow.borrow_mut();
        
        let mut panel = Panel::new_blank();
        
        panel.add_text_display(borrowed_control_flow.get_name());


        let condition_sub_panel = panel.add_sub_panel();
        if let Some(function) = borrowed_control_flow.get_function() {
            condition_sub_panel.add_widget(function.construct_widget());
        }

        ControlFlowWidget {
            panel,
        }
    }
    
    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::ScriptingElement(ScriptingElementWidget::ControlFlow(self))
    }

    pub fn get_panel(&self) -> &Panel {
        return &self.panel
    }

    pub fn get_mut_panel(&mut self) -> &mut Panel {    
        return &mut self.panel   
    }

}
