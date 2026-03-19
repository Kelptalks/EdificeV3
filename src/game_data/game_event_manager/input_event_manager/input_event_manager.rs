use miniquad::{KeyCode, MouseButton};

use crate::game_data::{game_event_manager::game_event_manager::game_event_manager::{GameEvent, GameEventManager}, screen::ScreenData};



#[derive(Clone)]
pub enum InputEvent {
    // Keys
    KeyDown(KeyCode, GameEvent),

    // Scrolling
    ScrollUp(GameEvent),
    ScrollDown(GameEvent),

    // Mouse button
    LeftMouseButtonClicked(GameEvent),
    RightMouseButtonClicked(GameEvent),

    LeftMouseButtonReleased(GameEvent),
    RightMouseButtonReleased(GameEvent),

}

impl InputEvent {
    pub fn execute_input_events(&self, event_data: &mut GameEventManager, screen_data: &ScreenData) {
        let input_manager = screen_data.get_input_manager();
        
        match self {
            InputEvent::KeyDown(key_code, event) => {
                // println!("{}", input_manager.was_key_code_pressed(*key_code));
                if input_manager.was_key_code_pressed(*key_code) {
                    event_data.add_game_event(event.clone());
                }
            },
            InputEvent::ScrollUp(event) => {
                if input_manager.get_mouse_input_data().scrolled_up() {
                    event_data.add_game_event(event.clone());
                }
            },
            InputEvent::ScrollDown(event) => {
                if input_manager.get_mouse_input_data().scrolled_down() {
                    event_data.add_game_event(event.clone());
                }
            },
            InputEvent::LeftMouseButtonClicked(event) => {
                if input_manager.get_mouse_input_data().was_left_clicked() {
                    event_data.add_game_event(event.clone());
                }
            },
            InputEvent::RightMouseButtonClicked(event) => {
                if input_manager.get_mouse_input_data().was_right_clicked() {
                    event_data.add_game_event(event.clone());
                }
            },
            InputEvent::LeftMouseButtonReleased(event) => {
                if input_manager.get_mouse_input_data().was_left_released() {
                    event_data.add_game_event(event.clone());
                }
            },
            InputEvent::RightMouseButtonReleased(event) => {
                if input_manager.get_mouse_input_data().was_left_released() {
                    event_data.add_game_event(event.clone());
                }
            },
        }
        

    }
}