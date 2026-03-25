use std::{cell::RefCell, rc::Rc};

use crate::game_data::game_event_manager::prelude::{Event, GameEvent, PrimEvent, WidgetEvent};

#[derive(Clone)]
pub enum UsizeEvent {
    ModUsize(Rc<RefCell<usize>>, i32),

}

impl UsizeEvent {
    pub fn wrap_into_event(self) -> Event {
        Event::GameEvent(GameEvent::WidgetEvent(WidgetEvent::PrimEvent(PrimEvent::UsizeEvent(self))))
    }

    pub fn wrap_into_event_vec(self) -> Vec<Event> {
        vec![self.wrap_into_event()]
    }

    pub fn execute(&self) {
        match self {
            UsizeEvent::ModUsize(usize_ref, mod_value) => {
                let current_usize = *usize_ref.borrow() as i32;

                let new_usize = current_usize + *mod_value;

                if new_usize < 0 {
                    *usize_ref.borrow_mut() = 0;
                }
                else {
                    *usize_ref.borrow_mut() = new_usize as usize;
                }

                 

            },
        }

    }
}