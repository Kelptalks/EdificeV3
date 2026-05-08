use crate::game_data::{game_event_manager::{event_manager::Event, game_event_manager::{GameEvent, GameEventManager}, render_event_manager::render_event_manager::RenderEvent}, player_data::player_data::PlayerData, screen::{screen_mananager::ScreenManager, widget::world_rendering::{tile_map::TileMapId, tile_map_manager::{self, TileMapManager}}}};

#[derive(Clone, PartialEq)]
pub enum TileMapEvent {
    FreeTileMap(TileMapId),
    DirtyTileMap(TileMapId)
}

impl TileMapEvent {
    pub fn wrap_into_event(self) -> Event {
        RenderEvent::TileMapEvent(self).wrap_into_event()
    }

    pub fn wrap_into_game_event(self) -> GameEvent {
        RenderEvent::TileMapEvent(self).wrap_into_game_event()
    }
    
    pub fn execute_event(
        &self, 
        tile_map_manager: &mut TileMapManager,

    ) -> Vec<Event> {
        match self {
            TileMapEvent::FreeTileMap(tile_map_id) => {
                return tile_map_manager.free_id(tile_map_id)
            },
            TileMapEvent::DirtyTileMap(tile_map_id) => {
                if let Some(tile_map) = tile_map_manager.get_mut_tile_map(*tile_map_id) {
                    tile_map.ray_casting_dirty = true;
                    tile_map.cashed_texture_dirty = true;
                }
            },
        }
        Vec::new()
    }
}