use crate::game_data::{TextureManager, screen::{ScreenData, screen_data, widget::panel::Panel}, texture_manager};

pub trait Widget {
    
    fn get_pos(&self) -> [f32; 4];
    fn get_scale(&self) -> [f32; 2];


    fn set_internal_buffers(&mut self, buffer: [f32; 4]);


    fn render(&self, texture_manager: &mut TextureManager, screen_data: &ScreenData);
}


pub enum WidgetType {
    Panel(Panel)
}

impl Widget for WidgetType {
    fn get_pos(&self) -> [f32; 4] {
        match self {
            WidgetType::Panel(w) => w.get_pos(),
        }
    }

    fn get_scale(&self) -> [f32; 2] {
        match self {
            WidgetType::Panel(w) => w.get_scale(),
        }
    }

    fn render(&self, texture_manager: &mut TextureManager, screen_data: &ScreenData) {
        match self {
            WidgetType::Panel(w) => w.render(texture_manager, screen_data),
        }
    }
    
    fn set_internal_buffers(&mut self, buffer: [f32; 4]) {
        match self {
            WidgetType::Panel(w) => w.set_internal_buffers(buffer),
        }
    }

}

impl WidgetType {
    pub fn new_panel(parent_pos: [f32; 4], side_buffers: [f32; 4]) -> Self {
        return WidgetType::Panel(Panel::new(parent_pos, side_buffers));
    }
}