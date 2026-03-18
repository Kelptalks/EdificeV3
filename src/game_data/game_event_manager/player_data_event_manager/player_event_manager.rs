use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::{game_event_manager::EventData, player_data_event_manager::location_event::LocationEvent}, player_data::{locations::location::WorldLocation, player_data::PlayerData}};

#[derive(Clone)]
pub enum PlayerDataEvent {
    LocationEvent(Rc<RefCell<WorldLocation>>, LocationEvent),
}


impl PlayerDataEvent {
    pub fn execute_player_data_events(&self, event_tools: &mut EventData, player_data: &mut PlayerData) {
        match self {
            PlayerDataEvent::LocationEvent(location_ref, location_event) => {
                location_event.execute_location_events(event_tools, location_ref.clone());
            },
        }
    }
}