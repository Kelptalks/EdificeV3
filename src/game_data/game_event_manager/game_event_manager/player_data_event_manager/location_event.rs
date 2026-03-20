use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::game_event_manager::game_event_manager::GameEventManager, player_data::locations::location::WorldLocation};

#[derive(Clone)]
pub enum LocationEvent {
    ShiftLocation([i32; 3]), // Shift Amount
    ShiftPoint(usize, [i32; 3]), // Point Index, Shift Amount

    SetLocationPoint(usize, [i32; 3]), // Point Index
    ModSize([i32; 3], bool) // Expand Directions, Expand
}


impl LocationEvent {
    pub fn execute_location_events(&self, event_tools: &mut GameEventManager, location_ref: Rc<RefCell<WorldLocation>>) {
        match self {
            LocationEvent::ShiftLocation(shift_cords) => {
                let mut location = location_ref.borrow_mut();
                location.get_mut_area().shift_cords(*shift_cords);
            },
            LocationEvent::ShiftPoint(point_index, shift_cords) => {
                let mut location = location_ref.borrow_mut();
                location.get_mut_area().shift_point(*point_index, *shift_cords);
            },
            LocationEvent::SetLocationPoint(index, new_point) => {
                let mut location = location_ref.borrow_mut();
                location.get_mut_area().set_point(*index, *new_point);
            },
            LocationEvent::ModSize(mod_scale, expand) => {
                let mut location = location_ref.borrow_mut();
                location.get_mut_area().resize(*mod_scale, *expand);
            },
        }


    }
}