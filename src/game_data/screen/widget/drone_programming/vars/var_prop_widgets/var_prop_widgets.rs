use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_script::var::var_properties::{PropValue, VarPropModRequest}, screen::widget::{drone_programming::vars::var_prop_widgets::{invintory_display_prop_widget::InvintoryDisplayPropWidget, num_prop_widget::NumDisplayPropWidget, text_display_prop_widget::TextDisplayPropWidget, var_prop_widget::VarPropWidget}, panel::panel::Panel, widget::{Widget, WidgetType}}};

pub enum VarPropVal {
    String(TextDisplayPropWidget),
    Num(NumDisplayPropWidget),
    Invintory(InvintoryDisplayPropWidget),
    Var(VarPropWidget)
}


impl VarPropVal {
    pub fn update_with_val(&mut self, val: PropValue) -> Vec<VarPropModRequest>{
        match self {
            VarPropVal::String(string_prop_widget) => {
                string_prop_widget.update_with_val(val)
            },
            VarPropVal::Num(w) => w.update_with_val(val),
            VarPropVal::Invintory(w) => w.update_with_val(val),
            VarPropVal::Var(w) => w.update_with_val(val),
        }
    }

    fn get_root_mut_panel(&mut self) -> &mut Panel {
        match self {
            VarPropVal::String(w) => w.get_root_mut_panel(),
            VarPropVal::Num(w) => w.get_root_mut_panel(),
            VarPropVal::Invintory(w) => w.get_root_mut_panel(),
            VarPropVal::Var(w) => w.get_root_mut_panel(),
        }
    }

    fn get_root_panel(&self) -> &Panel {
        match self {
            VarPropVal::String(w) => w.get_root_panel(),
            VarPropVal::Num(w) => w.get_root_panel(),
            VarPropVal::Invintory(w) => w.get_root_panel(),
            VarPropVal::Var(w) => w.get_root_panel(),
        }
    }
}

impl Widget for VarPropVal {
    fn get_pos(&self) -> [f32; 4] {
        self.get_root_panel().get_pos()
    }

    fn get_scale(&self) -> [f32; 2] {
        self.get_root_panel().get_scale()
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        self.get_root_panel().get_preffered_scale()
    }

    fn set_buffers(&mut self, pos: [f32; 4]) {
        self.get_root_mut_panel().set_buffers(pos);
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.get_root_mut_panel().set_parent_pos(pos);
    }

    fn size(&mut self) {
        self.get_root_mut_panel().size();
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager
    ) {
        self.get_root_mut_panel().render(texture_manager, screen_data, game_event_manager);
    }
}