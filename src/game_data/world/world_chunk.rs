use std::collections::HashMap;

const CHUNK_SIZE: usize = 64;
const CHUNK_AREA: usize = CHUNK_SIZE * CHUNK_SIZE;
const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

const CHUNK_SIZE_I32: i32 = 64;

pub struct WorldChunk {
    cords : [i16; 3],
    block_data : Box<[u16; CHUNK_VOLUME]>


    
    
}

impl WorldChunk {
    pub fn new(cords :[i16; 3]) -> Self
    {
        Self {
            cords : cords,
            block_data : Box::new([0; CHUNK_VOLUME])
        }
    }

    pub fn get_chunk_size_i32() -> i32 {
        CHUNK_SIZE_I32
    }

    // Convert the cords to index
    pub fn cords_to_index(cords :[usize; 3]) -> usize
    {
        let cord_index = cords[0] + (cords[1] * CHUNK_SIZE) + (cords[2] * CHUNK_AREA);
        if cord_index >= CHUNK_VOLUME
        {
            println!("World_Error : failed to set chunk value at cords({}, {}, {}) giving index({}) out of range", cords[0], cords[1], cords[2], cord_index);
            return 0;
        }
        return cord_index;
    }

    // Set the value of at a cord in a chunk
    pub fn set_chunk_value(&mut self, value : u16, cords :[usize; 3])
    {
        let cord_index = Self::cords_to_index(cords);
        self.block_data[cord_index] = value;
    }

    // Set the value of a cord in a chunk
    pub fn get_chunk_value(&self, cords :[usize; 3]) -> u16
    {
        let cord_index = Self::cords_to_index(cords);
        return self.block_data[cord_index];
    }

    pub fn test_world_chunk(){
        println!("Chunk Volume : {}", CHUNK_VOLUME);

        let mut world_chunk = Self::new([0, 0, 0]);
    
        for x_cor in 0..CHUNK_SIZE {
            println!("x_cor : {}", x_cor);
            for y_cor in 0..CHUNK_SIZE {
                for z_cor in 0..CHUNK_SIZE {
                    world_chunk.set_chunk_value(16, [x_cor, y_cor, z_cor]);
                }
            }
        }

        for x_cor in 0..CHUNK_SIZE {
            for y_cor in 0..CHUNK_SIZE {
                for z_cor in 0..CHUNK_SIZE {
                    if world_chunk.get_chunk_value([x_cor, y_cor, z_cor]) != 16
                    {
                        println!("Error Reading/Setting value in world chunk")
                    }
                }
            }
        }


    }

}