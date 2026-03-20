use miniquad::{KeyCode, MouseButton};

use crate::game_data::{game_event_manager::{game_event_manager::game_event_manager::{GameEvent, GameEventManager}, prelude::Event}, screen::ScreenData};



#[derive(Clone)]
pub enum InputEvent {
    // Keys
    KeyDown(KeyCode, Vec<Event>),

    // Scrolling
    ScrollUp(Vec<Event>),
    ScrollDown(Vec<Event>),

    // Mouse button
    LeftMouseButtonDown(Vec<Event>),
    RightMouseButtonDown(Vec<Event>),

    LeftMouseButtonReleased(Vec<Event>),
    RightMouseButtonReleased(Vec<Event>),

}

impl InputEvent {
    pub fn get_input_events_to_dispatch(&self, screen_data: &ScreenData) -> Vec<Event> {
        let input_manager = screen_data.get_input_manager();
        
        let mut events_to_dispatch = Vec::new();
        match self {
            InputEvent::KeyDown(key_code, events) => {
                // println!("{}", input_manager.was_key_code_pressed(*key_code));
                if input_manager.was_key_code_pressed(*key_code) {
                    events_to_dispatch.append(&mut events.clone());
                }
            },
            InputEvent::ScrollUp(events) => {
                if input_manager.get_mouse_input_data().scrolled_up() {
                    events_to_dispatch.append(&mut events.clone());
                }
            },
            InputEvent::ScrollDown(events) => {
                if input_manager.get_mouse_input_data().scrolled_down() {
                    events_to_dispatch.append(&mut events.clone());
                }
            },
            InputEvent::LeftMouseButtonDown(events) => {
                if input_manager.get_mouse_input_data().was_left_clicked() {
                    events_to_dispatch.append(&mut events.clone());
                }
            },
            InputEvent::RightMouseButtonDown(events) => {
                if input_manager.get_mouse_input_data().was_right_clicked() {
                    events_to_dispatch.append(&mut events.clone());
                }
            },
            InputEvent::LeftMouseButtonReleased(events) => {
                if input_manager.get_mouse_input_data().was_left_released() {
                    events_to_dispatch.append(&mut events.clone());
                }
            },
            InputEvent::RightMouseButtonReleased(events) => {
                if input_manager.get_mouse_input_data().was_left_released() {
                    events_to_dispatch.append(&mut events.clone());
                }
            },
        }
        
        return events_to_dispatch;

    }
}