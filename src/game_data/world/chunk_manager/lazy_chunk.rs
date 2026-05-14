use crate::game_data::World;

pub struct LazyWorldChunk {
    cords: [i16; 3]
}

impl LazyWorldChunk {
    pub fn new(cords: [i16; 3]) -> LazyWorldChunk {
        LazyWorldChunk {
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