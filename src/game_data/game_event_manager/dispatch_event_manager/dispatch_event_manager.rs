use std::{cell::RefCell, rc::Rc};

use crate::game_data::game_event_manager::event_manager::Event;

#[derive(Clone)]
pub enum DispatchEvent {
    DispatchEventList(Vec<Event>),
    IndexedEvent(Rc<RefCell<usize>>, Vec<Event>)
}

impl DispatchEvent {
    pub fn wrap_dispatch_event(self) -> Event {
        return Event::DispatchEvent(self);
    }

    pub fn get_events_dispatched(&self) -> Vec<Event> {
        let mut events_to_dispatch = Vec::new();
        match self {
            DispatchEvent::DispatchEventList(events) => {
                events_to_dispatch.append(&mut events.clone());
            },
            DispatchEvent::IndexedEvent(ref_index, events) => {
                events_to_dispatch.push(events[*ref_index.borrow()].clone());
            },
        }

        return events_to_dispatch;
    }
}