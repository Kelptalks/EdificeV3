use std::{cell::RefCell, fmt::format, rc::Rc};

use crate::game_data::{
    player_data::{
        drone_script::{
            control_flow, function::function::Function, script_element::ScriptElement, var::{programming_vars::programming_var::ProgrammingVar, var_type::VarType}}, drones::drone_actions::drone_actions::DroneAction}, screen::widget::{self, drone_programming::{control_flow_slot, scripting_widget_type::{self, ScriptingElementWidget, ScriptingWidget}}, panel::panel::{Panel, PanelAlignment, PanelOrientation}, prelude::{PanelColor, VarSlot}, widget::{Widget, WidgetType}, widget_calculations::TextSize, widget_properties::WidgetProperties}};



pub struct FunctionSlot {
    function_ref: Rc<RefCell<Function>>,
    panel: Panel,

    mouse_function_index: usize,
}

impl FunctionSlot {

    pub fn new_with_function(function_ref: &Rc<RefCell<Function>>) -> FunctionSlot {
        let mut panel = Panel::new_blank();
        let function_borrow = function_ref.borrow();


        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);


        panel.add_text_display(function_borrow.get_name()).set_text_scale(TextSize::Medium);


        let params = function_borrow.get_params();
        for param in params {
            let mut var_slot = VarSlot::new_with_var_ref(param.clone());
            var_slot.set_dragging_properties(true, true, true);
            panel.add_widget(var_slot.wrap_into_widget());
        }

        for (line, element) in function_borrow.get_body().iter().enumerate() {
           panel.add_widget(element.create_widget(line));
        }
        drop(function_borrow);

        panel.size();
        FunctionSlot {
            function_ref: function_ref.clone(),
            panel: panel,

            mouse_function_index: 0,
        }
    }

    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::FunctionSlot(self)
    }

    fn rebuild_widgets(&mut self) {
        let mut panel = Panel::new_blank();
        let function_borrow = self.function_ref.borrow();

        
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);


        panel.add_text_display(function_borrow.get_name()).set_text_scale(TextSize::ExtraSmall);

        let params = function_borrow.get_params();
        for param in params {
            let mut var_slot = VarSlot::new_with_var_ref(param.clone());
            var_slot.set_dragging_properties(true, true, true);
            panel.add_widget(var_slot.wrap_into_widget());
        }

        for (line, element) in function_borrow.get_body().iter().enumerate() {
           let mut widget = element.create_widget(line);
           if line == self.mouse_function_index {
            if let Some(scripting_widget) = &mut widget.as_scripting_widge() {
                scripting_widget.set_highlighted();
            }    
        }
           panel.add_widget(widget);
        }
        drop(function_borrow);

        panel.size();

        self.panel = panel;
    }



}

impl Widget for FunctionSlot {
    fn get_widget_properties(&self) -> &WidgetProperties {
        self.panel.get_widget_properties()
    }

    fn get_mut_widget_properties(&mut self) -> &mut WidgetProperties {
        self.panel.get_mut_widget_properties()
    }

    fn set_buffers(&mut self, pos: [f32; 4]) {
        self.panel.set_buffers(pos);
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.panel.set_parent_pos(pos);
    }

    fn size(&mut self) {
        self.panel.size();
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
    ) {
        self.panel.render(texture_manager, screen_data, game_event_manager);

        // Handle inputs
        if self.panel.is_mouse_on() {
            
            
            // get widget as scripting element widget
            let widget_mouse_is_on_option = self.panel.get_sub_widget_mouse_on(screen_data);
            
            
            let mut scripting_widget;
            if let Some(widget) = widget_mouse_is_on_option {
                scripting_widget = widget.as_scripting_widge();
            }
            else {
                if self.function_ref.borrow().get_body().len() == 0 {
                    scripting_widget = Some(ScriptingWidget::FunctionSlot())
                }
                else {
                    scripting_widget = None
                }
            }

            // REBUILD INPUTS  
            if let Some(scripting_widget) = &mut scripting_widget {                
                self.mouse_function_index = scripting_widget.get_line();
                
                // Remove element
                if screen_data.was_right_pressed() {
                    println!("Removing element at index({})", scripting_widget.get_line());    
                    self.function_ref.borrow_mut().remove_element(scripting_widget.get_line());
                }

                // Add element
                else if screen_data.was_left_released() {
                    let var_held_by_mouse = game_event_manager.get_mut_event_tools().get_mut_mouse_widget_data().get_var_held();
                    if let Some(var) = var_held_by_mouse {
                        let var_type_ref = var.get_var_type_ref();
                        let borrow = var_type_ref.borrow();
                        if let VarType::ProgrammingVar(ProgrammingVar::ScriptingElement(element)) = &*borrow {
                            
                            self.function_ref.borrow_mut().incert_element(scripting_widget.get_line(), element.clone());
                            
                            
                            println!("Adding script element: {}", element.get_name())
                        }
                    }
                }

                self.rebuild_widgets();
                self.size();
            }



            // NON REBUILD INPUTS




            
        }
        self.panel.set_color(PanelColor::Light);
    }
}


impl ScriptingElementWidget for FunctionSlot {
    fn get_line_index(&self) -> usize {
        0
    }
    
    
    fn set_highlighted(&mut self) {
        self.panel.set_color(PanelColor::Dark);
    }
}
