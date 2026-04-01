use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::prelude::{Event, GameEvent, PlayerDataEvent}, player_data::{drone_programming::var::var_type::Var, locations::location::WorldLocation}};




#[derive(Clone)]
pub enum VarEvents {
    ExpandWorldSize(Rc<RefCell<Var>>),



}

impl VarEvents {
    pub fn wrap_into_event(self) -> Event {
        return Event::GameEvent(GameEvent::PlayerDataEvent(PlayerDataEvent::VarEvent(self)));
    }

    pub fn execute(&self) {
        match self {
            _ => {
            
            }
        }
    }
}


