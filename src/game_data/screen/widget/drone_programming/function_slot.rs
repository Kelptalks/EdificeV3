#![allow(dead_code)]
use std::{cell::RefCell, rc::Rc};

use crate::game_data::{
    TextureManager, player_data::{
        drone_script::function::function::Function, player_data::PlayerData}, screen::{ScreenData, text::render_string_at_ndc, widget::{drone_programming::{
                script_element_body_slot::ScriptElementBodySlot, scripting_control_manager, scripting_widget_type::ScriptingElementWidget}, panel::panel::{Panel, PanelAlignment, PanelOrientation}, prelude::{PanelColor, VarSlot}, widget::{Widget, WidgetType}, widget_calculations::TextSize, widget_properties::WidgetProperties}}};



pub struct FunctionSlot {
    function_ref: Rc<RefCell<Function>>,
    panel: Panel,

    mouse_function_index: usize,
}

impl FunctionSlot {

    pub fn new_with_function(function_ref: &Rc<RefCell<Function>>) -> FunctionSlot {
        let panel = Panel::new_blank();



        let mut slot =FunctionSlot {
            function_ref: function_ref.clone(),
            panel: panel,

            mouse_function_index: 0,
        };

        slot.rebuild_widgets();

        slot


    }

    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::FunctionSlot(self)
    }

    fn rebuild_widgets(&mut self) {
        let saved_buffers = self.panel.get_widget_properties().external_buffers;
        let saved_parent_pos = self.panel.get_widget_properties().parent_pos;

        let mut panel = Panel::new_blank();
        let pause_button = panel.add_toggle_button();
        pause_button.set_icon(crate::game_data::types::UITextures::Pause);
        pause_button.set_text("Pause".to_string());

        


        let function_borrow = self.function_ref.borrow();

        
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::TopLeft);

        

        panel.add_text_display(function_borrow.get_name()).set_text_scale(TextSize::Medium);

        let params = function_borrow.get_params();
        for param in params {
            let mut var_slot = VarSlot::new_with_var(param.clone());
            var_slot.set_dragging_properties(true, true, true);
            panel.add_widget(var_slot.wrap_into_widget());
        }

        // Add body
        let mut body_slot = ScriptElementBodySlot::new(function_borrow.get_body());
        let mut step_key = function_borrow.get_execution_index_key().clone();
        if function_borrow.is_paused() {
            body_slot.highlight_key(&mut step_key, [255, 0, 0]);
        }
        else {
            body_slot.highlight_key(&mut step_key, [0, 255, 0]);
        }

        panel.add_widget(body_slot.wrap_into_widget());

        drop(function_borrow);

        panel.size();
        panel.set_parent_pos(saved_parent_pos);
        panel.set_buffers(saved_buffers);

        self.panel = panel;
    }


    fn render_debug_data(&mut self, texture_manager: &mut TextureManager, screen_data: &ScreenData) {
        let widget_mouse_on_option = self.panel.get_mut_sub_widget_mouse_on(screen_data);
        // Render key string
        if let Some(WidgetType::ScriptElementBodySlot(body_slot)) = widget_mouse_on_option {
            let index_keys = body_slot.get_mouse_index(screen_data);
            let key_string = format!("{:?}", index_keys);
            render_string_at_ndc(
                texture_manager, 
                key_string, 
                crate::game_data::types::FontType::Basic, 
                TextSize::Small.get_scale(), 
                screen_data.get_mouse_ndc()
            );  
        }
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
        player_data: &PlayerData,
    ) {
        self.panel.render(texture_manager, screen_data, event_manager, player_data);

        self.render_debug_data(texture_manager, screen_data);

        // Handle inputs
        if self.panel.mouse_on(screen_data) {
            // get widget as scripting element widget
            let widget_mouse_on_option = self.panel.get_mut_sub_widget_mouse_on(screen_data);

            if screen_data.was_left_pressed() {
                if let Some(WidgetType::Button(button)) = widget_mouse_on_option {
                    if button.mouse_on(screen_data) {
                        self.function_ref.borrow_mut().step_function();
                    }
                }
                else if let Some(WidgetType::ToggleButton(_button)) = widget_mouse_on_option {
                    self.function_ref.borrow_mut().toggle_pause();
                }
                else if let Some(WidgetType::ScriptElementBodySlot(body_slot)) = widget_mouse_on_option {
                    let index_keys = body_slot.get_mouse_index(screen_data);
                    self.function_ref.borrow_mut().set_execution_index_key(index_keys);
                    
                }
            }

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
            self.panel.set_color(PanelColor::SuperLightUI);
        }
        else {
            self.panel.set_color(PanelColor::LightUI);
        }
        self.rebuild_widgets();
        self.size();
    }
    
}


impl ScriptingElementWidget for FunctionSlot {
    fn get_line_index(&self) -> usize {
        0
    }
    
    
    fn highlight(&mut self, color: [u8; 3]) {
        self.panel.set_color(PanelColor::Custom(color[0], color[1], color[2]));
    }
    
    fn get_line_incert_index(&self, _screen_data: &ScreenData) -> usize {
        0
    }
}
