use crate::game_data::{TextureManager, screen::{ScreenData, screen_data}, texture_manager};

pub trait Widget {
    fn new(parent_pos: [f32; 4], side_buffers: [f32; 4]) -> Self;
    
    fn get_pos(&self) -> [f32; 4];
    fn get_scale(&self) -> [f32; 2];



    fn render(&self, texture_manager: &mut TextureManager, screen_data: &ScreenData);
}