use std::{cell::RefCell, rc::Rc};

use crate::game_data::{player_data::player_data::PlayerData, screen::{text::render_string_at_ndc, widget::{widget::{Widget, WidgetType}, widget_calculations::TextSize, widget_properties::WidgetProperties}}, types::FontType};

pub struct TextDisplay {
    widget_properties: WidgetProperties,

    prefered_char_scale: f32,

    string_ref: Rc<RefCell<String>>,
}

impl TextDisplay {
    pub fn new(text: String) -> TextDisplay {
        let char_scale = TextSize::ExtraSmall.get_scale();

        let mut wp = WidgetProperties::new_blank();
        wp.prefered_scale = [char_scale * text.len() as f32, char_scale];

        TextDisplay {
            widget_properties: wp,
            prefered_char_scale: char_scale,
            string_ref: Rc::new(RefCell::new(text)),
        }
    }

    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::TextDisplay(self)
    }

    pub fn get_string_ref(&self) -> &Rc<RefCell<String>> {
        return &self.string_ref;
    }

    pub fn set_string_ref(&mut self, string_ref: &Rc<RefCell<String>>) {
        self.string_ref = string_ref.clone();
    }

    pub fn set_text_scale(&mut self, size: TextSize) {
        self.prefered_char_scale = size.get_scale();
        self.size();
    }

    pub fn set_text(&mut self, text: String) {
        *self.string_ref.borrow_mut() = text;
    }

    pub fn size(&mut self) {
        self.widget_properties.scale_based_off_parent();

        let char_scale = self.prefered_char_scale;
        self.widget_properties.prefered_scale = [
            char_scale * self.string_ref.borrow().len() as f32,
            char_scale,
        ];
    }

    pub fn get_char_scale(&self) -> f32 {
        return self.prefered_char_scale;
    }

}


impl Widget for TextDisplay {
    fn get_widget_properties(&self) -> &WidgetProperties {
        &self.widget_properties
    }

    fn get_mut_widget_properties(&mut self) -> &mut WidgetProperties {
        &mut self.widget_properties
    }

    fn set_buffers(&mut self, buffers: [f32; 4]) {
        self.widget_properties.external_buffers = buffers;
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.widget_properties.parent_pos = pos;
    }

    fn size(&mut self) {
        self.size();
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        _screen_data: &crate::game_data::screen::ScreenData,
        _game_event_manager: &mut crate::game_data::game_event_manager::game_event_manager::EventManager,
        _player_data: &PlayerData,
    ) {
        let pos = self.widget_properties.pos;
        render_string_at_ndc(texture_manager,
            self.string_ref.borrow().clone(),
            FontType::Basic,
            self.widget_properties.scale[1],
            [pos[0], pos[1]]
        );
    }
}
