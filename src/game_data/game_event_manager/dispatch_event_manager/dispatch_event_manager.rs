use std::{cell::RefCell, rc::Rc};

use crate::game_data::game_event_manager::game_event_manager::{Event, GameEvent, GameEventManager};

pub enum DispatchEvent {
    IndexedEvent(Rc<RefCell<usize>>, Vec<Event>)
}

impl DispatchEvent {
    pub fn execute_input_events(&self, event_data: &mut GameEventManager) {
        match self {
            DispatchEvent::IndexedEvent(index, items) => todo!(),
        }
    }
}