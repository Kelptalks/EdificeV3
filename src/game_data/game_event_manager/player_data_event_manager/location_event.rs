use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::game_event_manager::EventData, player_data::locations::location::WorldLocation};

#[derive(Clone)]
pub enum LocationEvent {
    ShiftLocation([i32; 3]),
}


impl LocationEvent {
    pub fn execute_location_events(&self, event_tools: &mut EventData, location_ref: Rc<RefCell<WorldLocation>>) {
        match self {
            LocationEvent::ShiftLocation(shift_cords) => {
                let mut location = location_ref.borrow_mut();
                location.get_mut_area().shift_cords(*shift_cords);
            },
        }


    }
}