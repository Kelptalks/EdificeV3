use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::prelude::EventManager, player_data::drone_script::var::{var::Var, var_type::{VarType, VarTypeKind}}, screen::{ScreenData, text::render_string_at_ndc, widget::{widget::{Widget, WidgetType}, widget_calculations}}};



pub struct VarSlot {
    // Parent rendering
    parent_pos: [f32; 4],
    parent_scale: [f32; 2],
    prefered_scale: [f32; 2],

    // Self Rendering
    external_buffers: [f32; 4],  
    internal_buffers: [f32; 4],
    pos: [f32; 4],
    scale: [f32; 2],
    
    var: Var,
    var_string_ndc: [f32; 2],

    allow_setting: bool,
    allow_dragging: bool,
    allow_clearing: bool,
}

/*
#############
## VarSlot ##
#############

## Purpose
Var slots are ui Elements responsable for managing the usage of variables through
the actions of setting and getting them. 

## Usage
Var Slots contain an RC that can be used in events, or just to manage values
*/

impl VarSlot {
    pub fn new_with_var_type(var_type: VarType) -> VarSlot {

        VarSlot {
            // Parent Rendering
            parent_pos: [0.0; 4],
            parent_scale: [0.0; 2],
            prefered_scale: [widget_calculations::get_button_scale(); 2],

            // Self Rendering
            external_buffers: [0.0; 4], 
            internal_buffers: [0.012; 4],   
            pos: [0.0; 4],
            scale: [0.0; 2],


            var: Var::new_with_var_type(var_type),
            var_string_ndc: [0.0; 2],

            // Options
            allow_setting: true,
            allow_dragging: true,
            allow_clearing: true,
        }
    }

    pub fn wrap_into_widget(self) -> WidgetType {
        return WidgetType::VarSlot(self);
    }

    //=====================================
    // Input Propertys
    //=====================================

    pub fn set_dragging_properties(&mut self, allow_dragging: bool, allow_setting: bool, allow_clearing: bool) {
        self.allow_dragging = allow_dragging;
        self.allow_setting = allow_setting;
        self.allow_clearing = allow_clearing;
    }

    //=====================================
    // Control Managment Functions
    //=====================================


    // Set the variable of the slot if var released matches both type allowed and settings is allowed
    fn try_and_set_var(&mut self, var_held_by_mouse: &Option<Rc<RefCell<VarType>>>) {
        
        
        if self.allow_setting {
            if let Some(var_held_by_mouse) = var_held_by_mouse {
                let cloned_var_type = var_held_by_mouse.borrow().clone();
                self.var.set_value(cloned_var_type);
            }
        }
    }

    // Get the value of the slot ref if it allows
    fn try_and_get_var(&mut self) -> Option<Rc<RefCell<VarType>>> {
        if self.allow_dragging {
            return Some(self.var.get_var_type_ref().clone());
        }
        else {
            return None;
        }
    }

}

impl Widget for VarSlot {
    fn get_pos(&self) -> [f32; 4] {
        return self.pos;
    }

    fn get_scale(&self) -> [f32; 2] {
        return self.scale;
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        return self.prefered_scale;
    }

    fn set_buffers(&mut self, buffers: [f32; 4]) {
        self.external_buffers = buffers;
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.parent_pos = pos;
    }

    fn size(&mut self) {
        self.pos = widget_calculations::buffer_pos(self.parent_pos, self.external_buffers);
        self.scale = widget_calculations::pos_to_scale(self.pos);
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &ScreenData,
        game_event_manager: &mut EventManager
    ) {
        
        if self.allow_clearing {
            texture_manager.render_texture_with_pos(self.var.get_kind_texture(), self.pos);
        }
        
        texture_manager.render_texture_with_pos(self.var.get_texture(), self.pos);

        // Try and get var if mouse was released
        if screen_data.mouse_on_ndc_pos(self.pos) {

            // Size and set string
            let string = self.var.get_name();
            let text_scale = widget_calculations::get_button_text_scale();
            let string_centering_offset = (string.len() as f32 * text_scale) / 2.0;
            

            let string_ndc = [
                (self.pos[0] + self.scale[0] / 2.0) - string_centering_offset, 
                self.pos[1] - (self.scale[1] / 4.0),
                ];

            render_string_at_ndc(
                texture_manager, 
                string, 
                crate::game_data::types::FontType::Basic, 
                widget_calculations::TextSize::ExtraSmall.get_scale(), 
                string_ndc,
            );
    
            
            // Set
            if screen_data.was_left_released() {
                let var_held_by_mouse = game_event_manager.get_mut_event_tools().get_mut_mouse_widget_data().get_var_held();
                self.try_and_set_var(var_held_by_mouse);
            }

            // Get 
            if screen_data.was_left_pressed() {
                game_event_manager.get_mut_event_tools().get_mut_mouse_widget_data().set_var_held(self.try_and_get_var());
            }

            // Clear
            if self.allow_setting && screen_data.was_right_pressed() {
                self.var.clear();
            }
        }
    }
}