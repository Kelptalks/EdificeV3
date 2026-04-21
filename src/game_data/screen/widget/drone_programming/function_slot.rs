use std::{cell::RefCell, fmt::format, intrinsics::copy_nonoverlapping, rc::Rc};

use crate::game_data::{
    game_event_manager::{self, prelude::{EventManager, GameEventManager}}, player_data::{
        drone_script::{
            control_flow, 
            function::{self, function::Function}, 
            script_element::ScriptElement, 
            var::{programming_vars::programming_var::ProgrammingVar, var_type::VarType}
        }, drones::drone_actions::drone_actions::DroneAction}, 
        screen::{self, 
            ScreenData, 
            screen_data, 
            widget::{self, drone_programming::{
                control_flow_slot, script_element_body_slot::ScriptElementBodySlot, scripting_control_manager, scripting_widget_type::{self, ScriptingElementWidget, ScriptingWidgetType}}, panel::panel::{Panel, PanelAlignment, PanelOrientation}, prelude::{PanelColor, VarSlot}, widget::{Widget, WidgetType}, widget_calculations::TextSize, widget_properties::WidgetProperties}}};



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

        let body_slot = ScriptElementBodySlot::new(function_borrow.get_body());
        panel.add_widget(body_slot.wrap_into_widget());
        

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
        let saved_buffers = self.panel.get_widget_properties().external_buffers;
        let saved_parent_pos = self.panel.get_widget_properties().parent_pos;

        let mut panel = Panel::new_blank();
        let function_borrow = self.function_ref.borrow();

        
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);

        

        panel.add_text_display(function_borrow.get_name()).set_text_scale(TextSize::Medium);

        let params = function_borrow.get_params();
        for param in params {
            let mut var_slot = VarSlot::new_with_var_ref(param.clone());
            var_slot.set_dragging_properties(true, true, true);
            panel.add_widget(var_slot.wrap_into_widget());
        }

        // Add body
        let body_slot = ScriptElementBodySlot::new(function_borrow.get_body());
        panel.add_widget(body_slot.wrap_into_widget());

        drop(function_borrow);

        panel.size();
        panel.set_parent_pos(saved_parent_pos);
        panel.set_buffers(saved_buffers);

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
        event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
    ) {
        self.panel.render(texture_manager, screen_data, event_manager);

        // Handle inputs
        if self.panel.mouse_on(screen_data) {
            // get widget as scripting element widget
            
            let widget_mouse_on_option = self.panel.get_mut_sub_widget_mouse_on(screen_data);


            if screen_data.was_left_released() {
                if let Some(WidgetType::ScriptElementBodySlot(body_slot)) = widget_mouse_on_option {
            
                    let mut index_keys = body_slot.get_mouse_incert_index(screen_data);
                    
                    let mut borrow = self.function_ref.borrow_mut();
                    let body = borrow.get_mut_body();
                    
                    println!("{:?}", index_keys);

                    scripting_control_manager::handle_mouse_element_body_incert(
                        body, 
                        event_manager, 
                        &mut index_keys,
                    )
                    
                    
                    
                }
            }


            self.rebuild_widgets();
            self.panel.set_color(PanelColor::SuperLightUI);
        }
        else {
            self.panel.set_color(PanelColor::LightUI);
        }
    }
}


impl ScriptingElementWidget for FunctionSlot {
    fn get_line_index(&self) -> usize {
        0
    }
    
    
    fn set_highlighted(&mut self) {

    }
    
    fn get_line_incert_index(&self, screen_data: &ScreenData) -> usize {
        0
    }
}
