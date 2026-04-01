use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::prelude::EventManager, player_data::drone_programming::var::var_type::{Var, VarTypeKind}, screen::{ScreenData, text::render_string_at_ndc, widget::{widget::{Widget, WidgetType}, widget_calculations}}};



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
    
    var_ref: Rc<RefCell<Var>>,
    var_string_ndc: [f32; 2],


    var_type_kind_allowed: VarTypeKind,
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
    pub fn new(var_instance: &Rc<RefCell<Var>>) -> VarSlot {

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


            var_ref: var_instance.clone(),
            var_string_ndc: [0.0; 2],

            // Options
            var_type_kind_allowed: VarTypeKind::Any,
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

    pub fn set_allowed_type(&mut self, var_type_allowed: VarTypeKind) {
        self.var_type_kind_allowed = var_type_allowed;
    }


    //=====================================
    // Control Managment Functions
    //=====================================


    // Set the variable of the slot if var released matches both type allowed and settings is allowed
    fn try_and_set_var(&mut self, var_held_by_mouse: &Option<Rc<RefCell<Var>>>) {
        if self.allow_setting {
            if let Some(var_held_by_mouse) = var_held_by_mouse {
                // Make sure the var held by mouse is not the same as this var
                if var_held_by_mouse == &self.var_ref {
                    return;
                }
                else if var_held_by_mouse.borrow().to_kind() == self.var_type_kind_allowed {
                    *self.var_ref.borrow_mut() = var_held_by_mouse.borrow().clone();
                }
            }
        }
    }

    // Get the value of the slot ref if it allows
    fn try_and_get_var(&mut self) -> Option<Rc<RefCell<Var>>> {
        if self.allow_dragging {
            return Some(self.var_ref.clone());
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
        self.var_string_ndc = [self.pos[0] + self.scale[0] / 2.0, self.pos[1]];
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &ScreenData,
        game_event_manager: &mut EventManager
    ) {
        
        texture_manager.render_texture_with_pos(self.var_type_kind_allowed.get_texture(), self.pos);
        texture_manager.render_texture_with_pos(self.var_ref.borrow().get_texture(), self.pos);

        // Try and get var if mouse was released
        if screen_data.mouse_on_ndc_pos(self.pos) {

            render_string_at_ndc(
                texture_manager, 
                self.var_ref.borrow().get_name(), 
                crate::game_data::types::FontType::Basic, 
                widget_calculations::get_button_text_scale(), 
                screen_data.get_mouse_ndc(),
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
                self.var_ref.borrow_mut().clear();
            }
        }
    }
}