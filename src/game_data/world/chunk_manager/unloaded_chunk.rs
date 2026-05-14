use crate::game_data::{World, chunk_manager::chunk_manager::WorldChunkType};



pub struct UnloadedWorldChunk {
    pub load: bool,
    cords: [i16; 3],
}

impl UnloadedWorldChunk {
    
    pub fn wrap_into_chunk_type(self) -> WorldChunkType {
        WorldChunkType::Unloaded(self)
    }
    pub fn new(cords: [i16; 3]) -> UnloadedWorldChunk {
        UnloadedWorldChunk {
            load: false,
            cords,
        }
    }


    pub fn get_cords(&self) -> [i16; 3] {
        self.cords
    }

    pub fn get_key(&self) -> u64 {
        World::chunk_cords_to_key(self.cords)
    }
}