use crate::game_data::{World, game_event_manager::{event_manager::Event, game_event_manager::GameEventManager, world_event_manager::world_event_manager::WorldEvent}};

#[derive(Clone)]
pub enum WorldChunkEvent {
    LoadChunk([i16; 3]),
    DirtyChunk([i16; 3])
}

impl WorldChunkEvent {
    pub fn wrap_into_event(self) -> Event {
        WorldEvent::ChunkEvent(self).wrap_into_event()
    }

    pub fn execute_chunk_event(&self, world: &mut World, event_data: &mut GameEventManager) {

        match self {
            WorldChunkEvent::LoadChunk(cords) => {
                world.load_chunk(cords);
            },
            WorldChunkEvent::DirtyChunk(cords) => {
                world.dirty_chunk(cords);
            },
        }
    }
}