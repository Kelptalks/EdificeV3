use crate::game_data::game_event_manager::widget_event_manager::prim_events::{bool_events::BoolEvent, i32_event::I32Event, string_events::StringEvent, usize_events::UsizeEvent};

#[derive(Clone)]
pub enum PrimEvent {
    StringEvent(StringEvent),
    UsizeEvent(UsizeEvent),
    BoolEvent(BoolEvent),
    I32Event(I32Event),
    
}

impl PrimEvent {
    pub fn execute(&self) {
        match self {
            PrimEvent::StringEvent(string_event) => {
                string_event.execute();
            },
            PrimEvent::UsizeEvent(usize_event) => {
                usize_event.execute();
            },
            PrimEvent::BoolEvent(bool_event) => {
                bool_event.execute();
            },
            PrimEvent::I32Event(i32_event) => {
                i32_event.execute();
            },
        }
    }
}