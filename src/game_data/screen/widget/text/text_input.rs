use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::prelude::{Event, StringEvent, UsizeEvent}, screen::widget::{text::{header::TextDisplay, text_input_event_constructor}, widget::Widget}, texture_manager::texture::Texture};

pub struct TextInput {
    text_display: TextDisplay,
    
    current_index_ref: Rc<RefCell<usize>>,
    cursor_max_index: usize,

    events: Vec<Event>,

}


impl TextInput {
    pub fn new_text_input(string_ref: &Rc<RefCell<String>>) -> TextInput {
        let mut text_display = TextDisplay::new("".to_string());
        text_display.set_string_ref(&string_ref.clone());

        let cursor_index_ref = Rc::new(RefCell::new(0));


        TextInput {
            text_display: text_display,

            current_index_ref: cursor_index_ref.clone(),
            cursor_max_index: 30,

            events: text_input_event_constructor::get_text_inputs(string_ref, &cursor_index_ref)
        }

    }


    pub fn get_cursor_pos(&self) -> [f32; 4] {
        let text_pos = self.text_display.get_pos();

        let char_scale = self.text_display.get_char_scale();

        let cursor_start = char_scale * *self.current_index_ref.borrow() as f32;

        return [
            text_pos[0] + cursor_start,
            text_pos[1],
            text_pos[0] + cursor_start + char_scale,
            text_pos[3],
        ];
    }

    pub fn set_max_index(&mut self, new_max: usize) {
        self.cursor_max_index = new_max;
    }
}

impl Widget for TextInput {
    fn get_pos(&self) -> [f32; 4] {
        self.text_display.get_pos()
    }

    fn get_scale(&self) -> [f32; 2] {
        self.text_display.get_scale()
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        self.text_display.get_preffered_scale()
    }

    fn set_buffers(&mut self, pos: [f32; 4]) {
        self.text_display.set_buffers(pos);
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.text_display.set_parent_pos(pos);
    }

    fn size(&mut self) {
        self.text_display.size();
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager
    ) {
        self.text_display.size();
        self.text_display.render(texture_manager, screen_data, game_event_manager);

        // Make sure curosor is in string bounds
        let string_len = self.text_display.get_string_ref().borrow().len();
        let cursor_index = *self.current_index_ref.borrow();
        if string_len < cursor_index {
            *self.current_index_ref.borrow_mut() = string_len;
        }

        texture_manager.render_texture_with_pos(Texture::UITexture(crate::game_data::types::UITextures::ButtonCircle), self.get_cursor_pos());


        game_event_manager.add_events(&self.events);

        // Backspace if cursor index is over max
        if cursor_index > self.cursor_max_index {
            let mut backspace_event = 
            StringEvent::RemoveCharWithRefIndex(
                self.text_display.get_string_ref().clone(), self.current_index_ref.clone()
            ).wrap_into_event_vec();

            backspace_event.insert(1, UsizeEvent::ModUsize(self.current_index_ref.clone(), -1).wrap_into_event());
            game_event_manager.add_events(&backspace_event);
        }
    }
}

