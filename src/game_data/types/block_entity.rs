use crate::game_data::game_event_manager::prelude::{GameEvent, PlayerDataEvent};

pub enum EntityType {
    Drone,
    Puff,
    
}

impl EntityType {
    pub fn to_creation_event(&self, cords: [i32; 3]) -> GameEvent {
        match self {
            EntityType::Drone => GameEvent::PlayerDataEvent(PlayerDataEvent::CreateDrone(cords)),
            EntityType::Puff => todo!(),
        }
    }
}