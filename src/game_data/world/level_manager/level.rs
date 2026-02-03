use crate::game_data::{World, screen::Button};

// Trait names should be PascalCase
pub trait Level {
    fn get_name(&self) -> String;
    fn gen_level(&self, world: &mut World);
    fn get_level_select_button(&self) -> Button;
}