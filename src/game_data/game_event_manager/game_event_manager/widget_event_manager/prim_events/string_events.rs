use std::{cell::RefCell, rc::Rc};

use crate::game_data::game_event_manager::{prelude::{Event, GameEvent, WidgetEvent}, widget_event_manager::prim_events::prim_event_manager::PrimEvent};

#[derive(Clone)]
pub enum StringEvent {
    Clear(Rc<RefCell<String>>),
    InsertCharWithRefIndex(Rc<RefCell<String>>, Rc<RefCell<usize>>, char),
    RemoveCharWithRefIndex(Rc<RefCell<String>>, Rc<RefCell<usize>>),
}

impl StringEvent {

    pub fn wrap_into_event(self) -> Event {
        Event::GameEvent(GameEvent::WidgetEvent(WidgetEvent::PrimEvent(PrimEvent::StringEvent(self))))
    }

    pub fn wrap_into_event_vec(self) -> Vec<Event> {
        vec![self.wrap_into_event()]
    }

    pub fn execute(&self) {
        match self {
            StringEvent::Clear(string_ref) => {
                string_ref.borrow_mut().clear()
            }
            StringEvent::InsertCharWithRefIndex(string_ref, index_ref, char) => {
                let index = *index_ref.borrow();
                let mut string = string_ref.borrow_mut();
                
                println!("index: {}", index);
                println!("string_len: {}", string.len());

                if index <= string.len() {
                    string.insert(index, *char);
                }
            }
            StringEvent::RemoveCharWithRefIndex(string_ref, index_ref) => {
                let index = *index_ref.borrow();
                let mut string = string_ref.borrow_mut();
                if index < string.len() {
                    string.remove(index);
                }
            }
        }
    }
}