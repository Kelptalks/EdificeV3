use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::{game_event_manager::game_event_manager::GameEventManager, prelude::{Event, GameEvent, PlayerDataEvent}}, player_data::{drone_programming::var::{self, game_vars::{dynamic_var::{self}, game_var_type::GameVar}, var_type::Var}, locations::location::WorldLocation}};

#[derive(Clone)]
pub enum LocationEvent {
    ShiftLocation([i32; 3]), // Shift Amount
    ShiftPoint(usize, [i32; 3]), // Point Index, Shift Amount

    SetLocationPoint(usize, [i32; 3]), // Point Index
    ModSize([i32; 3], bool), // Expand Directions, Expand

    SetAreaWithVarRef(Rc<RefCell<Var>>),
}


impl LocationEvent {
    pub fn wrap_into_event(self, location_ref: Rc<RefCell<WorldLocation>>) -> Event {
        return Event::GameEvent(GameEvent::PlayerDataEvent(PlayerDataEvent::LocationEvent(location_ref, self)));
    }

    pub fn execute(&self, event_tools: &mut GameEventManager, location_ref: Rc<RefCell<WorldLocation>>) {
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
            LocationEvent::SetAreaWithVarRef(var_ref) => {
                let borrowed = var_ref.borrow();
                if let Var::Game(GameVar::Dynamic(dynamic_var_ref)) = &*borrowed {
                    if let Some(area) = dynamic_var_ref.get_area() {
                        location_ref.borrow_mut().set_area(area);
                    }
                }

            }
        }


    }
}