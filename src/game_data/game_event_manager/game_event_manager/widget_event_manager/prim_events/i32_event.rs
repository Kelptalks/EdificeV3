use std::{cell::RefCell, rc::Rc};

use crate::game_data::game_event_manager::prelude::{Event, PrimEvent, WidgetEvent};

#[derive(Clone)]
pub enum I32Event {
    mod_i32(Rc<RefCell<i32>>, i32),
}



impl I32Event {

    pub fn wrap_into_event(self) -> Event {
        WidgetEvent::PrimEvent(PrimEvent::I32Event(self)).wrap_into_event()
    }

    pub fn wrap_into_event_vec(self) -> Vec<Event> {
        vec![self.wrap_into_event()]
    }

    pub fn execute(&self) {
        match self {
            I32Event::mod_i32(ref_cell, amount) => {
                *ref_cell.borrow_mut() += amount;
            },
        }
    }
}