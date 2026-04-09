use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::drone_programming::var::var_properties::{PropKey, PropValue, VarPropModRequest}, screen::widget::{drone_programming::vars::var_prop_widgets::var_prop_widgets::VarPropVal, panel::panel::Panel, text::text_input::TextInput, widget::{Widget, WidgetType}}};

pub struct TextDisplayPropWidget {
    
    // Mutable
    mutable: bool,
    input_focused_ref: Option<Rc<RefCell<bool>>>,
    
    key: PropKey,
    panel: Panel,
    string_ref: Rc<RefCell<String>>,
}

impl TextDisplayPropWidget {
    pub fn new(key: PropKey, mutable: bool) -> TextDisplayPropWidget {
        
        let mut panel = Panel::new_blank();
        
        // Key name 
        let key_name = format!("{}: ", key.to_name());
        panel.add_text_display(key_name);

        let string_ref;
        let input_focused_ref;
        if mutable {
            string_ref = Rc::new(RefCell::new(" ".to_string()));
            let mut text_input = TextInput::new_text_input(&string_ref);
            input_focused_ref = Some(text_input.get_focused_ref().clone());
            panel.add_widget(text_input.wrap_into_widget());
        }
        else {
            let text_display = panel.add_text_display("String Prop Widget".to_string());
            string_ref = text_display.get_string_ref().clone();
            input_focused_ref = None;
        }
        
    

        
        TextDisplayPropWidget {
            mutable,
            input_focused_ref,

            key,
            panel,
            string_ref
        }
    }


    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::VarPropValWidget(VarPropVal::String(self))
    }


    pub fn update_with_val(&mut self, val: PropValue) -> Vec<VarPropModRequest> {
        let mut prop_requests = Vec::new();
        
        // Don't update if the text input is focused on
        if self.mutable {
            if let Some(focused) = &self.input_focused_ref {
                if *focused.borrow() {
                    prop_requests.push(VarPropModRequest::Set(self.key, PropValue::String(self.string_ref.borrow().clone())));
                    return prop_requests;
                }
            }
        }
        

        *self.string_ref.borrow_mut() = val.into_string();
        self.panel.size();

        return prop_requests;
    }






    
}



impl Widget for TextDisplayPropWidget {
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
        self.panel.set_parent_pos(pos);
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
        self.panel.render(texture_manager, screen_data, game_event_manager);
    }
}
