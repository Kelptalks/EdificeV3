

use std::{cell::RefCell, rc::Rc};

use crate::game_data::{
    player_data::drone_programming::{
        function::{function::Function, function_return_value::FunctionReturnValue}, 
        script_element, 
        var::{programming_vars::programming_var::ProgrammingVar, var_type::Var}
    }, 
    screen::widget::{
        panel::panel::{Panel, PanelAlignment, PanelOrientation}, prelude::{PanelColor, VarSlot}, text::header::TextDisplay, widget::{Widget, WidgetType}, widget_calculations::TextSize}
    };

pub struct FunctionWidget {
    panel: Panel,

    function_ref: Rc<RefCell<Function>>
}



impl FunctionWidget {
    pub fn new(function: &Rc<RefCell<Function>>) -> FunctionWidget {

        let mut function_widget = FunctionWidget {
            panel: Panel::new_blank(),
            function_ref: function.clone()
        };

        function_widget.rebuild();

        function_widget
    }

    pub fn get_panel(&self) -> &Panel {
        return &self.panel
    }

    pub fn get_mut_panel(&mut self) -> &mut Panel {    
        return &mut self.panel   
    }

    //=====================================
    // Construction
    //=====================================

    pub fn rebuild(&mut self) {
        let borrowed_function = self.function_ref.borrow();
         
        let mut panel = Panel::new_blank();
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);
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
            let mut script_element_panel = panel.add_sub_panel();
            script_element_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);

            let text_display = TextDisplay::new("Body".to_string());
            script_element_panel.add_widget(text_display.wrap_into_widget());

            for script_element in script_elements {
                script_element_panel.add_widget(script_element.construct_widget());
            } 
        }

        // Add Return Functions
        let return_functions = borrowed_function.get_return_functions();
        if return_functions.len() == 0 {
            let return_panel = panel.add_sub_panel();
            return_panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);
            return_panel.add_text_display("return".to_string()).set_text_scale(TextSize::ExtraSmall);
            
            for return_function in return_functions {
                return_panel.add_widget(return_function.construct_widget());
            }
        }

        self.panel = panel;
    }


    //=====================================
    // Controls
    //=====================================

    fn handle_released_var(&mut self, var_held_by_mouse: &Rc<RefCell<Var>>) {
        let borrow = var_held_by_mouse.borrow_mut();
        if let Var::ProgrammingVar(programming_var) = &*borrow {
            if let ProgrammingVar::Function(function) = programming_var{
                self.function_ref.borrow_mut().add_script_element(0, function.clone().to_script_element());
                
                self.rebuild();    

                self.panel.set_color(PanelColor::Dark);
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
                    game_event_manager.get_mut_event_tools().get_mut_mouse_widget_data().release_var_held();
                }
            }

        }   
    }
}

impl Widget for FunctionWidget {
    fn get_pos(&self) -> [f32; 4] {
        self.panel.get_pos()
    }

    fn get_scale(&self) -> [f32; 2] {
        self.panel.get_scale()
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        self.panel.get_preffered_scale()
    }

    fn set_buffers(&mut self, pos: [f32; 4]) {
        self.panel.set_buffers(pos)
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.panel.set_parent_pos(pos)
    }

    fn size(&mut self) {
        self.panel.size();
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager
    ) {

        // 
        // 
        // 
        
        // Render first to handle lowest level input 
        self.panel.render(texture_manager, screen_data, game_event_manager);
        
        self.handle_inputs(screen_data, game_event_manager);
        
    }
}