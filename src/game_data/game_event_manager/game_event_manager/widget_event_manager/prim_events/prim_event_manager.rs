use crate::game_data::game_event_manager::widget_event_manager::prim_events::{bool_events::BoolEvent, string_events::StringEvent, usize_events::UsizeEvent};

#[derive(Clone)]
pub enum PrimEvent {
    StringEvent(StringEvent),
    UsizeEvent(UsizeEvent),
    BoolEvent(BoolEvent),
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
        }
    }
}