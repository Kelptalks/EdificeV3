use std::{collections::HashMap, hash::Hash, path::absolute, ptr::null, sync::Arc};

use crate::game_data::{log_header, log_init, tik_manager::drones::drone_manager::DroneManager, types::BlockType, world::world_gen::WorldGenManager};


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

    // Convert the cords to index
    pub fn cords_to_index(cords :[usize; 3]) -> usize
    {
        let cord_index = cords[0] + (cords[1] * CHUNK_SIZE) + (cords[2] * CHUNK_AREA);
        if (cord_index >= CHUNK_VOLUME)
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
                    if (world_chunk.get_chunk_value([x_cor, y_cor, z_cor]) != 16)
                    {
                        println!("Error Reading/Setting value in world chunk")
                    }
                }
            }
        }


    }

}

pub struct World {
    loaded_chunks : HashMap<u64, WorldChunk>,
    total_chunks : u32,
    
}

impl World {
    pub fn new() -> Self
    {
        log_init("Creating world");
        
        Self { 
            loaded_chunks: HashMap::new(),
            total_chunks: 0,
        }
    }


    //=====================================
    // Conversion Funcions
    //=====================================

    pub fn chunk_cords_to_key(cords : [i16 ; 3]) -> u64
    {
        // Cast through u16 to preserve bit pattern without sign extension
        let x = cords[0] as u16 as u64;
        let y = cords[1] as u16 as u64;
        let z = cords[2] as u16 as u64;
        
        return (z << 32) | (y << 16) | x
    }

    pub fn world_cords_to_chunk_cords(cords : [i32 ; 3]) -> [i16; 3]
    {
        let chunk_size = CHUNK_SIZE_I32;
    
        // Use div_euclid for proper floor division
        let chunk_x = cords[0].div_euclid(chunk_size) as i16;
        let chunk_y = cords[1].div_euclid(chunk_size) as i16;
        let chunk_z = cords[2].div_euclid(chunk_size) as i16;
        
        [chunk_x, chunk_y, chunk_z]
    }


    //=====================================
    // Getters / Setters
    //=====================================

    // Get a chunk at chunk cords or create it if it doesn't exist
    // Optimization Note : Could use Entry API to elimanate one hashmap lookup.
    pub fn get_chunk_at_chunk_cords_mut(&mut self, cords : [i16 ; 3] ) -> &mut WorldChunk {
        let chunk_key = Self::chunk_cords_to_key(cords);
        
        // If world chunk does not exist create it
        if !self.loaded_chunks.contains_key(&chunk_key) 
        {
            let new_chunk = WorldChunk::new(cords);
            self.loaded_chunks.insert(chunk_key, new_chunk);
        }

        return self.loaded_chunks.get_mut(&chunk_key).unwrap();
    }

    // Gets the chunk if it exists 
    pub fn get_chunk_at_chunk_cords(&self, cords : [i16 ; 3] ) -> Option<&WorldChunk> {
        let chunk_key = Self::chunk_cords_to_key(cords);
        return self.loaded_chunks.get(&chunk_key);
    }

    pub fn get_chunk_at_world_cords_mut(&mut self, cords : [i32 ; 3]) -> &mut WorldChunk
    {
        let chunk_cords = Self::world_cords_to_chunk_cords(cords);
        return self.get_chunk_at_chunk_cords_mut(chunk_cords);
    }

    pub fn get_chunk_at_world_cords(&self, cords : [i32 ; 3]) -> Option<&WorldChunk>
    {
        let chunk_cords = Self::world_cords_to_chunk_cords(cords);
        return self.get_chunk_at_chunk_cords(chunk_cords);
    }

    // Get the modded world cords that give the local chunk cords
    pub fn world_cords_to_internal_chunk_cords(cords : [i32 ; 3]) -> [usize; 3]
    {
        // Mod cords to get internal chunk cords 
        let chunk_size = CHUNK_SIZE_I32;
        // Use rem_euclid for proper modulo that handles negatives correctly
        let x_moded_cord = cords[0].rem_euclid(chunk_size) as usize;
        let y_moded_cord = cords[1].rem_euclid(chunk_size) as usize;
        let z_moded_cord = cords[2].rem_euclid(chunk_size) as usize;


        return [x_moded_cord, y_moded_cord, z_moded_cord];
    }

    pub fn set_world_value (&mut self, value : u16, cords : [i32 ; 3]) {
        let world_chunk = self.get_chunk_at_world_cords_mut(cords);
        let internal_chunk_cords = Self::world_cords_to_internal_chunk_cords(cords);

        world_chunk.set_chunk_value(value, internal_chunk_cords);  
    }

    pub fn get_world_value (&self, cords : [i32 ; 3]) -> u16
    {
        let world_chunk = self.get_chunk_at_world_cords(cords);
        let internal_chunk_cords = Self::world_cords_to_internal_chunk_cords(cords);

        if world_chunk.is_some()
        {
            let chunk = world_chunk.unwrap();
            return chunk.get_chunk_value(internal_chunk_cords);
        }
        else
        {
            return 0;    
        }
    }

    pub fn get_arc_ref(self) -> Arc<World> {
        return Arc::new(self);
    }


    //=====================================
    // Terrain Gen
    //=====================================

    pub fn generate_terrain(&mut self, size: u32) {
        let size = size as i32 / 2;
        let start_cords = [-size, -size, -25];
        let end_cords = [size, size, 25];

        let world_gen_manager = WorldGenManager::new();
        world_gen_manager.generate_area(self, start_cords, end_cords);

    }

    //=====================================
    // Drones
    //=====================================

    pub fn get_drone_data() {

    }

    //=====================================
    // Tiking
    //=====================================

    pub fn tik_world(&mut self){
        
    }

    pub fn test_world()
    {
        let mut world = World::new();

        for x_cor in -1000..1000
        {
            world.set_world_value(10, [x_cor, 0, x_cor]);
        }

        for x_cor in -1000..1000
        {
            if world.get_world_value([x_cor, 0, x_cor]) != 10 
            {
                println!("somthings broken")
            }
        }
        
    }


}