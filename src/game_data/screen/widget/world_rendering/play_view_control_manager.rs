use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::prelude::*, player_data::locations::location::{WorldLocation}};


fn construct_shift_event(
    shift_type_ref: &Rc<RefCell<usize>>, 
    location_ref: &Rc<RefCell<WorldLocation>>, 
    shift: [i32; 3]
) -> Event {

    // Create shift events 
    let default_shift_event = 
        PlayerDataEvent::LocationEvent(
            location_ref.clone(), 
            LocationEvent::ShiftLocation(shift)
        ).wrap_into_event();


    let mouse_held_shift_event = 
        PlayerDataEvent::LocationEvent(
            location_ref.clone(),
            LocationEvent::ShiftPoint(0, shift)
        ).wrap_into_event();

    
    // Wrap with event type
    let event_shift_list = vec![
        default_shift_event, 
        mouse_held_shift_event
        ];

    let dispatch_control_flow_event = 
        DispatchEvent::IndexedEvent(
            shift_type_ref.clone(), 
            event_shift_list
        ).wrap_dispatch_event();

    
    return dispatch_control_flow_event;
}



pub struct PlayViewControlManager {
    shift_type_ref : Rc<RefCell<usize>>,
    location_ref: Rc<RefCell<WorldLocation>>,
}

impl PlayViewControlManager {
    pub fn new(shift_type_ref: &Rc<RefCell<usize>>, location_ref: &Rc<RefCell<WorldLocation>>) -> PlayViewControlManager{
        PlayViewControlManager {
            shift_type_ref: shift_type_ref.clone(),
            location_ref: location_ref.clone()
        }
    }

    pub fn get_camera_shift_event(&self, shift: [i32; 3]) -> Event {
        return self::construct_shift_event(&self.shift_type_ref, &self.location_ref, shift);
    }
    
}