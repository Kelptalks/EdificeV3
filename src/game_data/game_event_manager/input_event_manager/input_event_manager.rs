#![allow(dead_code)]
use miniquad::KeyCode;

use crate::game_data::{game_event_manager::prelude::Event, screen::ScreenData};



#[derive(Clone)]
pub enum InputEvent {
    // Keys
    KeyDown(KeyCode, Vec<Event>),
    KeysDown(Vec<KeyCode>, Vec<Event>),

    // Scrolling
    ScrollUp(Vec<Event>),
    ScrollDown(Vec<Event>),

    // Mouse button
    LeftMouseButtonDown(Vec<Event>),
    RightMouseButtonDown(Vec<Event>),

    LeftMouseButtonReleased(Vec<Event>),
    RightMouseButtonUp(Vec<Event>),

}

pub fn construct_key_down_event(key_code: KeyCode, event: Event) -> Event {
    return InputEvent::KeyDown(key_code, vec![event]).wrap_into_event();
}

impl InputEvent {
    pub fn wrap_into_event(self) -> Event {
        return Event::InputEvent(self);
    }

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
            InputEvent::KeysDown(key_codes, events) => {
                for key_code in key_codes {
                    if !input_manager.was_key_code_pressed(*key_code) {
                        return vec![];
                    }
                    events_to_dispatch.append(&mut events.clone());
                }
                
                events_to_dispatch.append(&mut events.clone());
                
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
            InputEvent::RightMouseButtonUp(events) => {
                if input_manager.get_mouse_input_data().was_right_released() {
                    events_to_dispatch.append(&mut events.clone());
                }
            },
        }
        
        return events_to_dispatch;

    }
}