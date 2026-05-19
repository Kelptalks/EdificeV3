use crate::game_data::player_data::{game_entity::game_entity_manager::GameEntityId, locations::location_manager::LocationId};


#[derive(Clone, Copy)]
pub enum ViewMode {
    God(),
    GameObjectSpectate(GameEntityId),
    Location(LocationId),
}
