

use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_programming::{function::{function::Function, function_return_value::FunctionReturnValue}, script_element, var::{programming_vars::programming_var::ProgrammingVar, var_type::Var}}, screen::widget::{drone_programming::scripting_elements::scripting_element_widget_manager::ScriptingElementWidget, panel::panel::{Panel, PanelAlignment, PanelOrientation}, prelude::VarSlot, widget::{Widget, WidgetType}, widget_calculations::TextSize}};

pub struct FunctionWidget {
    panel: Panel,


    function_ref: Rc<RefCell<Function>>
}



impl FunctionWidget {
    pub fn new(function: &Rc<RefCell<Function>>) -> FunctionWidget {
        let borrowed_function = function.borrow_mut();
        
        let mut panel = Panel::new_blank();
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
        panel.add_text_display(borrowed_function.get_name());
        

        // Add Function Params
        let parames = borrowed_function.get_params();
        if parames.len() != 0 {
            let param_panel = panel.add_sub_panel();
            param_panel.add_text_display("params".to_string()).set_text_scale(TextSize::ExtraSmall);
            for param in parames {
                let slot = VarSlot::new(param);
                param_panel.add_widget(slot.wrap_into_widget());
            }
        }

        // Add Function Body
        let script_elements = borrowed_function.get_script_elements();
        if script_elements.len() != 0 {
            let script_element_panel = panel.add_sub_panel();
            script_element_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
            script_element_panel.add_text_display("Body".to_string()).set_text_scale(TextSize::ExtraSmall);
            
            for script_element in script_elements {
                script_element_panel.add_widget(script_element.construct_widget());
            } 
        }

        // Add Return Functions
        let return_functions = borrowed_function.get_return_functions();
            if return_functions.len() != 0 {
            let return_panel = panel.add_sub_panel();
            return_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);
            return_panel.add_text_display("return".to_string()).set_text_scale(TextSize::ExtraSmall);
            
            for return_function in return_functions {
                return_panel.add_widget(return_function.construct_widget());
            }
        }

        FunctionWidget {
            panel,
            function_ref: function.clone()
        }
    }
    

    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::ScriptingElement(ScriptingElementWidget::Function(self))
    }

    pub fn get_panel(&self) -> &Panel {
        return &self.panel
    }

    pub fn get_mut_panel(&mut self) -> &mut Panel {    
        return &mut self.panel   
    }

     fn handle_released_var(&mut self, var_held_by_mouse: &Rc<RefCell<Var>>) {
        let borrow = var_held_by_mouse.borrow_mut();
        if let Var::ProgrammingVar(programming_var) = &*borrow {
            if let ProgrammingVar::Function(function) = programming_var{
                self.function_ref.borrow_mut().add_script_element(0, function.clone().to_script_element());
                println!("test");
            }
            else if let ProgrammingVar::Condition(condition) = programming_var {

            }
        }
    }

    pub fn handle_inputs(
        &mut self, 
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager)
    {
        if screen_data.mouse_on_ndc_pos(self.panel.get_pos()) {
            // Add var
            if screen_data.was_left_released() {
                let var_held_by_mouse = game_event_manager.get_mut_event_tools().get_mut_mouse_widget_data().get_var_held();
                if let Some(var) = var_held_by_mouse {
                    self.handle_released_var(var);
                }
            }

        }   
    }
}