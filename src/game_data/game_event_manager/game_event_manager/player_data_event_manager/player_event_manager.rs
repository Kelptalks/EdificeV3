use std::{cell::RefCell, rc::Rc};

use crate::game_data::{game_event_manager::{game_event_manager::{game_event_manager::GameEventManager, player_data_event_manager::location_event::LocationEvent}, prelude::{Event, GameEvent}}, player_data::{locations::{location::WorldLocation, location_config::WorldLocationConfig}, player_data::PlayerData}};

#[derive(Clone)]
pub enum PlayerDataEvent {
    CreateLocation(Rc<RefCell<WorldLocationConfig>>),
    LocationEvent(Rc<RefCell<WorldLocation>>, LocationEvent),
}


impl PlayerDataEvent {
    pub fn wrap_into_event(self) -> Event {
        return Event::GameEvent(GameEvent::PlayerDataEvent(self));
    }

    pub fn execute_player_data_events(&self, event_tools: &mut GameEventManager, player_data: &mut PlayerData) {
        match self {
            PlayerDataEvent::LocationEvent(location_ref, location_event) => {
                location_event.execute_location_events(event_tools, location_ref.clone());
            },
            PlayerDataEvent::CreateLocation(location_config) => {
                let location_area = location_config.borrow().get_source_location_ref().borrow().get_area().clone();
                let location_name = location_config.borrow().get_location_name();

                player_data.get_mut_location_manager().create_location(location_name, location_area);
            },
        }
    }
}