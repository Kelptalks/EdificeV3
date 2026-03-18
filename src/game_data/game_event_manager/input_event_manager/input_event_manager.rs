use miniquad::KeyCode;

use crate::game_data::{game_event_manager::game_event_manager::{Event, EventData}, screen::ScreenData};



#[derive(Clone)]
pub enum InputEvent {
    KeyDown(KeyCode, Event),
}

impl InputEvent {
    pub fn execute_input_events(&self, event_data: &mut EventData, screen_data: &ScreenData) {
        let input_manager = screen_data.get_input_manager();
        
        match self {
            InputEvent::KeyDown(key_code, event) => {
                // println!("{}", input_manager.was_key_code_pressed(*key_code));
                if input_manager.was_key_code_pressed(*key_code) {
                    event_data.add_game_event(event.clone());
                }
            },
        }
        

    }
}