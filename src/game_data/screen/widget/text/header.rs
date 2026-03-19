use crate::game_data::{screen::{text::render_string_at_ndc, widget::{widget::Widget, widget_calculations::{self, TextSize}}}, types::FontType};

pub struct TextDisplay {
    // Parent rendering
    parent_pos: [f32; 4],
    parent_scale: [f32; 2],
    prefered_scale: [f32; 2],
    prefered_char_scale: f32,

    // Self
    text: String,
    pos: [f32; 4],
    scale: [f32; 2],
    external_buffers: [f32; 4],

}

impl TextDisplay {
    pub fn new(text: String) -> TextDisplay {
        TextDisplay {
            // Parent
            parent_pos: [0.0; 4],
            parent_scale: [0.0; 2],
            prefered_scale: [0.0; 2],
            prefered_char_scale: TextSize::Small.get_scale(),

            // Self Rendering
            text: text,
            pos: [0.0; 4],
            scale: [0.0; 2],
            external_buffers: [0.0; 4],
            
        }
    }

    pub fn set_text_scale(&mut self, size: TextSize) {
        self.prefered_char_scale = size.get_scale();
    }

    pub fn size(&mut self) {
        self.pos = widget_calculations::buffer_pos(self.parent_pos, self.external_buffers);
        self.scale = widget_calculations::pos_to_scale(self.pos);
        
        let char_scale =  self.prefered_char_scale;

        self.prefered_scale = [
            char_scale * self.text.len() as f32,
            char_scale
        ];
    }


}


impl Widget for TextDisplay {
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
        self.size();
    }

    fn render(
        &mut self, 
        texture_manager: &mut crate::game_data::TextureManager, 
        screen_data: &crate::game_data::screen::ScreenData, 
        game_event_manager: &mut crate::game_data::game_event_manager::game_event_manager::EventManager
    ) {
        render_string_at_ndc(texture_manager, 
            self.text.clone(), 
            FontType::Basic, 
            self.scale[1], 
            [self.pos[0], self.pos[1]]
        );
    }
}