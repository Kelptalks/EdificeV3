use std::{cell::RefCell, rc::Rc};

use miniquad::KeyCode;

use crate::game_data::{game_event_manager::prelude::{BoolEvent, Event, InputEvent, StringEvent, UsizeEvent}, screen::widget::{panel::panel_texture_manager::PanelTextureManager, text::{header::TextDisplay, text_input_event_constructor}, widget::{Widget, WidgetType}}, texture_manager::texture::Texture};

pub struct TextInput {
    pos: [f32; 4],
    scale: [f32; 2],

    text_display: TextDisplay,
    
    focused_bool_ref: Rc<RefCell<bool>>,

    current_index_ref: Rc<RefCell<usize>>,
    max_string_size: usize,

    events: Vec<Event>,
    panel_texture: PanelTextureManager,
}


impl TextInput {
    pub fn wrap_into_widget(self) -> WidgetType {
        WidgetType::TextInput(self)
    }

    pub fn new_text_input(string_ref: &Rc<RefCell<String>>) -> TextInput {

        // Init text_display with max size for propper positioning
        let mut text_display = TextDisplay::new(" ".to_string()); 
        text_display.set_string_ref(&string_ref.clone());


        let cursor_index_ref = Rc::new(RefCell::new(0));

        // Setup Inputs
        let input_events = text_input_event_constructor::get_text_inputs(string_ref, &cursor_index_ref);
        let focused_bool_ref = Rc::new(RefCell::new(false));


        TextInput {
            pos: [0.0; 4],
            scale: [0.0; 2],

            text_display: text_display,

            focused_bool_ref,

            current_index_ref: cursor_index_ref.clone(),
            max_string_size: 30,

            events: input_events,
            panel_texture: PanelTextureManager::new(),
        }

    }


    pub fn get_cursor_pos(&self) -> [f32; 4] {
        let text_pos = self.text_display.get_pos();

        let char_scale = self.text_display.get_scale()[1];

        let cursor_start = char_scale * *self.current_index_ref.borrow() as f32;

        return [
            text_pos[0] + cursor_start,
            text_pos[1],
            text_pos[0] + cursor_start + char_scale,
            text_pos[3],
        ];
    }

    pub fn set_cursor_to_last_char(&self) {
        let mut current_cursor_index = 0;
        for (i, char) in self.text_display.get_string_ref().borrow().chars().enumerate() {
            if char != ' ' {
                current_cursor_index = i + 1;
            }
        }
        *self.current_index_ref.borrow_mut() = current_cursor_index;
    }

    pub fn get_focused_ref(&self) -> &Rc<RefCell<bool>> {
        return &self.focused_bool_ref;
    }
}

impl Widget for TextInput {
    fn get_pos(&self) -> [f32; 4] {
        self.pos
    }

    fn get_scale(&self) -> [f32; 2] {
        self.scale
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        let char_scale = self.text_display.get_char_scale();
        [char_scale * self.max_string_size as f32, char_scale]
    }

    fn set_buffers(&mut self, pos: [f32; 4]) {
        self.text_display.set_buffers(pos);
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.text_display.set_parent_pos(pos);
    }

    fn size(&mut self) {
        self.text_display.size();
        let char_scale = self.text_display.get_scale()[1];
        self.pos = self.text_display.get_pos();
        self.scale = [char_scale * self.max_string_size as f32, char_scale];
        self.panel_texture.size(self.pos, self.scale);
    }

    fn render(
        &mut self,
        texture_manager: &mut crate::game_data::TextureManager,
        screen_data: &crate::game_data::screen::ScreenData,
        game_event_manager: &mut crate::game_data::game_event_manager::prelude::EventManager,
        bounds: Option<[f32; 4]>,
    ) {
        
        self.panel_texture.render(texture_manager, bounds);
        self.text_display.render(texture_manager, screen_data, game_event_manager, bounds);
        

        // If not focused on
        if screen_data.mouse_on_ndc_pos(self.get_pos()) {
            if screen_data.get_input_manager().get_mouse_input_data().was_left_clicked() {
                self.panel_texture.set_color(crate::game_data::screen::widget::panel::panel_color::PanelColor::Dark);
                *self.focused_bool_ref.borrow_mut() = true;
                self.set_cursor_to_last_char();
            }
        }
        else {
            if screen_data.get_input_manager().get_mouse_input_data().was_left_clicked() {
                *self.focused_bool_ref.borrow_mut() = false;
            }
        }

        // If focused on 
        if *self.focused_bool_ref.borrow() {
            // Make sure curosor is in string bounds
            let string_len = self.text_display.get_string_ref().borrow().len();
            let cursor_index = *self.current_index_ref.borrow();
            if string_len < cursor_index {
                *self.current_index_ref.borrow_mut() = string_len;
            }

            // Exit of escape or enter was pressed
            if screen_data.get_input_manager().was_key_code_pressed(KeyCode::Enter) 
            || screen_data.get_input_manager().was_key_code_pressed(KeyCode::Escape)
            {
                *self.focused_bool_ref.borrow_mut() = false;
            }

            texture_manager.render_texture(Texture::UITexture(crate::game_data::types::UITextures::ButtonCircle), self.get_cursor_pos());


            game_event_manager.add_events(&self.events);

            // Backspace if cursor index is over max
            if cursor_index > self.max_string_size {
                let mut backspace_event = 
                StringEvent::RemoveCharWithRefIndex(
                    self.text_display.get_string_ref().clone(), self.current_index_ref.clone()
                ).wrap_into_event_vec();

                backspace_event.insert(1, UsizeEvent::ModUsize(self.current_index_ref.clone(), -1).wrap_into_event());
                game_event_manager.add_events(&backspace_event);
            }

            // Trim the string to max cursor index length
            let mut string = self.text_display.get_string_ref().borrow_mut();
            string.truncate(self.max_string_size);
        }
        else {
            self.panel_texture.set_color(crate::game_data::screen::widget::panel::panel_color::PanelColor::Light);
        }
    }
}