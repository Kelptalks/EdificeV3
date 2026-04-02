use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::prelude::{Event, GameEvent, PlayerDataEvent}, locations::world_area_side::WorldAreaSide, player_data::{drone_programming::var::{game_vars::{dynamic_var::{self, DynamicVar}, game_var_type::GameVar}, var_type::Var}, locations::location::{self, WorldLocation}}};




#[derive(Clone)]
pub enum VarEvents {
    LocationVarEvent(Rc<RefCell<Var>>, LocationVarEvent),



}


impl VarEvents {
    pub fn wrap_into_event(self) -> Event {
        return Event::GameEvent(GameEvent::PlayerDataEvent(PlayerDataEvent::VarEvent(self)));
    }

    pub fn wrap_into_event_vec(self) -> Vec<Event> {
        return vec![self.wrap_into_event()];
    }

    pub fn execute(&self) {
        match self {
            VarEvents::LocationVarEvent(var_ref, location_var_event) => {
                let borrow = var_ref.borrow();

                if let Var::Game(GameVar::Dynamic(DynamicVar::Location(location_option_ref))) = &*borrow {
                    if let Some(location) = location_option_ref {
                        location_var_event.execute(location);
                    }
                    else {
                        eprint!("Cannot execute Location Var Event on NULL location");
                    }
                }
                else {
                    eprint!("Cannot execute Location event on var type: {}", borrow.get_name());
                }
            }
        }
    }
}


#[derive(Clone)]
pub enum LocationVarEvent {
    ExpandLocationSide(WorldAreaSide),
    ShrinkLocationSide(WorldAreaSide),
}

impl LocationVarEvent {

    pub fn execute(&self, location: &Rc<RefCell<WorldLocation>>) { 
        match self {
            LocationVarEvent::ExpandLocationSide(world_area_side) => {
                location.borrow_mut().get_mut_area().expand(world_area_side);
            },
            LocationVarEvent::ShrinkLocationSide(world_area_side) => {
                location.borrow_mut().get_mut_area().shrink(world_area_side);
            },
        }
    }
}