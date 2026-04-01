use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::prelude::{Event, GameEvent, GameEventManager, PlayerDataEvent}, player_data::drones::drone::Drone};

#[derive(Clone)]
pub enum DroneEvent {
    Temp(),

}

impl DroneEvent {
    pub fn wrap_into_event(self, drone_ref: &Rc<RefCell<Drone>>) -> Event {
        return Event::GameEvent(GameEvent::PlayerDataEvent(PlayerDataEvent::DroneEvent(drone_ref.clone(), self)));
    }

    pub fn execute(&self, event_tools: &mut GameEventManager, drone_ref: &Rc<RefCell<Drone>>) {
        match self {
            DroneEvent::Temp() => {
                
            },
        }

    }
}