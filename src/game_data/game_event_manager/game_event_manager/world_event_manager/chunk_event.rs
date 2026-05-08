use crate::game_data::{World, game_event_manager::game_event_manager::GameEventManager};

#[derive(Clone)]
pub enum ChunkEvent {
    LoadChunk([i16; 3])
}

impl ChunkEvent {
    pub fn execute_chunk_event(&self, world: &mut World, event_data: &mut GameEventManager) {

        match self {
            ChunkEvent::LoadChunk(cords) => {
                world.load_chunk(cords);
            },
        }
    }
}