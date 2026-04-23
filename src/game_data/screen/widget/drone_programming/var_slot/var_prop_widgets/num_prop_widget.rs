use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::{prelude::{Event, PrimEvent}, widget_event_manager::prim_events::i32_event::I32Event}, player_data::drone_script::var::{prim_vars::prim_var_type::PrimitiveVarType, var::Var, var_properties::{PropKey, PropValue, VarPropModRequest}, var_type::VarType}, screen::widget::{drone_programming::var_slot::var_prop_widgets::var_prop_widgets::VarPropVal, panel::panel::{Panel, PanelAlignment, PanelOrientation}, text::header::TextDisplay, widget::{Widget, WidgetType}}};

pub struct NumDisplayPropWidget {
    // Mutable
    mutable: bool,
    
    key: PropKey,
    panel: Panel,
    
    num_ref: Rc<RefCell<i32>>,
    string_ref: Rc<RefCell<String>>,
}


impl NumDisplayPropWidget {



    pub fn new(key: PropKey, mutable: bool) -> NumDisplayPropWidget {
        let mut panel = Panel::new_blank();
        
        panel.set_orientation(PanelOrientation::Vertical, PanelAlignment::Center);

        let key_name = format!("{}", key.to_name());
        panel.add_text_display(key_name);

        let text_display = TextDisplay::new(" ".to_string());
        let string_ref = text_display.get_string_ref().clone();
        let num_ref = Rc::new(RefCell::new(0));

        panel.add_widget(text_display.wrap_into_widget());

        if mutable {
            let button_panel = panel.add_sub_panel();
            let add_button = button_panel.add_button();
            add_button.add_left_click_event(I32Event::mod_i32(num_ref.clone(), 1).wrap_into_event());
            add_button.add_right_click_event(I32Event::mod_i32(num_ref.clone(), -1).wrap_into_event());
            add_button.set_icon(crate::game_data::types::UITextures::ModIcon);
        }
        else {

        }

        NumDisplayPropWidget {
            mutable: mutable,

            key: key,
            panel: panel,
            num_ref: num_ref,

            string_ref: string_ref,
        }
    }

    pub fn update_with_var(&mut self, var: &Var) -> Vec<VarPropModRequest> {
        let mut prop_requests = Vec::new();

        if let Some(prop_num) = var.as_i32() {
            
            // get new prop value
            let dif = *self.num_ref.borrow();
            if dif != 0 {
                let new_value = dif + prop_num;
                prop_requests.push(VarPropModRequest::Set(self.key, PrimitiveVarType::Num(new_value).create_var()));

                // Reset
                *self.num_ref.borrow_mut() = 0;
            }
        }

        
        if let Some(string) = var.as_string() {
            *self.string_ref.borrow_mut() = string;
        }
        else {
            eprintln!("Cannot convert var from key{} to string in num display widget", self.key.to_name())
        }

        return prop_requests;
    }


    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::VarPropValWidget(VarPropVal::Num(self))
    }

    pub fn get_root_mut_panel(&mut self) -> &mut Panel {
        return &mut self.panel;
    }

    pub fn get_root_panel(&self) -> &Panel {
        return &self.panel;
    }

}


impl Widget for NumDisplayPropWidget {
    fn get_widget_properties(&self) -> &crate::game_data::screen::widget::widget_properties::WidgetProperties {
        self.panel.get_widget_properties()
    }

    fn get_mut_widget_properties(&mut self) -> &mut crate::game_data::screen::widget::widget_properties::WidgetProperties {
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
    }
}