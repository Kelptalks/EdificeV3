use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_programming::var::var_properties::{PropValue, VarPropRequest}, screen::widget::{drone_programming::vars::var_prop_widgets::text_display_prop_widget::TextDisplayPropWidget, widget::{Widget, WidgetType}}};

pub enum VarPropVal {
    String(TextDisplayPropWidget),

}


impl VarPropVal {
    pub fn update_with_val(&mut self, val: PropValue) -> Vec<VarPropRequest>{
        match self {
            VarPropVal::String(string_prop_widget) => {
                string_prop_widget.update_with_val(val)
            },
        }
    }
}

impl Widget for VarPropVal {
    fn get_pos(&self) -> [f32; 4] {
        match self {
            VarPropVal::String(w) => w.get_pos(),
        }
    }

    fn get_scale(&self) -> [f32; 2] {
        match self {
            VarPropVal::String(w) => w.get_scale(),
        }
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        match self {
            VarPropVal::String(w) => w.get_preffered_scale(),
        }
    }

    fn set_buffers(&mut self, pos: [f32; 4]) {
        match self {
            VarPropVal::String(w) => w.set_buffers(pos),
        }
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        match self {
            VarPropVal::String(w) => w.set_parent_pos(pos),
        }
    }

    fn size(&mut self) {
        match self {
            VarPropVal::String(w) => w.size(),
        }
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager
    ) {
        match self {
            VarPropVal::String(string_prop_widget) => string_prop_widget.render(texture_manager, screen_data, game_event_manager),
        }
    }
}