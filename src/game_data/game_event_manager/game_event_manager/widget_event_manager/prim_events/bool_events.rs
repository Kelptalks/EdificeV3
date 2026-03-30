use std::{cell::RefCell, rc::Rc};

use crate::game_data::game_event_manager::prelude::{Event, GameEvent, PrimEvent, WidgetEvent};

#[derive(Clone)]
pub enum BoolEvent {
    SetBool(Rc<RefCell<bool>>, bool),
    ToggleBool(Rc<RefCell<bool>>),
}

impl BoolEvent {
    pub fn wrap_into_event(self) -> Event {
        Event::GameEvent(GameEvent::WidgetEvent(WidgetEvent::PrimEvent(PrimEvent::BoolEvent(self))))
    }

    pub fn wrap_into_event_vec(self) -> Vec<Event> {
        vec![self.wrap_into_event()]
    }

    pub fn execute(&self) {
        match self {
            BoolEvent::SetBool(bool_ref, bool) => {
                *bool_ref.borrow_mut() = *bool;
            },
            BoolEvent::ToggleBool(bool_ref) => {
                let toggled_value = !*bool_ref.borrow();
                *bool_ref.borrow_mut() = toggled_value;
            },
        }

    }
}