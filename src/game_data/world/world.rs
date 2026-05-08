use std::collections::HashMap;
use std::fmt::format;

use mlua::Chunk;

use crate::game_data::game_event_manager::event_manager::{self, Event, EventManager};
use crate::game_data::player_data::player_data::PlayerData;
use crate::game_data::world_gen::WorldGenManager;
use crate::game_data::{TextureManager};
use crate::game_data::screen::widget::world_rendering::tile_map_manager::TileMapManager;
use crate::game_data::{types::BlockTexture, world_chunk::WorldChunk};
use crate::game_data::world::world_chunk::CHUNK_VOLUME;




pub struct World {
    loaded_chunks : HashMap<u64, WorldChunk>,
    
    chunks_to_load: Vec<u64>,
    chunks_to_generate: Vec<u64>,

    total_chunks : u32,

    world_gen_manager: WorldGenManager,
}

impl World {
    pub fn new() -> Self
    {   
        Self { 
            loaded_chunks: HashMap::new(),
            chunks_to_load: Vec::new(),
            chunks_to_generate: Vec::new(),

            total_chunks: 0,

            world_gen_manager: WorldGenManager::new()
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

    pub fn key_to_chunk_cords(key: u64) -> [i16; 3] {
        let x = (key & 0xFFFF) as u16 as i16;
        let y = ((key >> 16) & 0xFFFF) as u16 as i16;
        let z = ((key >> 32) & 0xFFFF) as u16 as i16;
        [x, y, z]
    }

    pub fn world_cords_to_chunk_cords(cords : [i32 ; 3]) -> [i16; 3]
    {
        let chunk_size = WorldChunk::get_chunk_size_i32();
    
        // Use div_euclid for proper floor division
        let chunk_x = cords[0].div_euclid(chunk_size) as i16;
        let chunk_y = cords[1].div_euclid(chunk_size) as i16;
        let chunk_z = cords[2].div_euclid(chunk_size) as i16;
        
        [chunk_x, chunk_y, chunk_z]
    }


    //=====================================
    // Chunk Getters
    //=====================================

    // Get a chunk at chunk cords
    pub fn force_get_chunk_at_cords_mut(&mut self, cords : [i16 ; 3] ) -> &mut WorldChunk {
        
        let chunk_key = Self::chunk_cords_to_key(cords);

        // If world chunk does not exist create it
        if !self.loaded_chunks.contains_key(&chunk_key) {
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

    pub fn force_get_chunk_at_world_cords_mut(&mut self, cords : [i32 ; 3]) -> &mut WorldChunk {
        let chunk_cords = Self::world_cords_to_chunk_cords(cords);
        return self.force_get_chunk_at_cords_mut(chunk_cords);
    }
    pub fn get_chunk_at_world_cords(&self, cords : [i32 ; 3]) -> Option<&WorldChunk> {
        let chunk_cords = Self::world_cords_to_chunk_cords(cords);
        return self.get_chunk_at_chunk_cords(chunk_cords);
    }


    pub fn set_chunk_block_data(&mut self, chunk_cords: [i16; 3], block_data: Box<[u16; CHUNK_VOLUME]>) {
        let chunk = self.force_get_chunk_at_cords_mut(chunk_cords);
        chunk.set_block_data(block_data);
        
    }

    pub fn create_chunk(&mut self, key: u64) {
        let cords = Self::key_to_chunk_cords(key);
        let world_chunk = WorldChunk::new(cords);
        self.chunks_to_generate.push(key);

        self.loaded_chunks.insert(key, world_chunk);
    }

    //=====================================
    // Single Block
    //=====================================

    // Get the modded world cords that give the local chunk cords
    pub fn world_cords_to_internal_chunk_cords(cords : [i32 ; 3]) -> [usize; 3]
    {
        // Mod cords to get internal chunk cords 
        let chunk_size = WorldChunk::get_chunk_size_i32();
        // Use rem_euclid for proper modulo that handles negatives correctly
        let x_moded_cord = cords[0].rem_euclid(chunk_size) as usize;
        let y_moded_cord = cords[1].rem_euclid(chunk_size) as usize;
        let z_moded_cord = cords[2].rem_euclid(chunk_size) as usize;


        return [x_moded_cord, y_moded_cord, z_moded_cord];
    }

    pub fn set_world_value (&mut self, value : u16, cords : [i32 ; 3]) {
        let world_chunk = self.force_get_chunk_at_world_cords_mut(cords);
        let internal_chunk_cords = Self::world_cords_to_internal_chunk_cords(cords);

        world_chunk.set_chunk_value(value, internal_chunk_cords)
    }

    pub fn get_world_value (&self, cords : [i32 ; 3]) -> u16
    {
        let world_chunk = self.get_chunk_at_world_cords(cords);

        if let Some(world_chunk) = world_chunk {
            let internal_chunk_cords = Self::world_cords_to_internal_chunk_cords(cords);
            return world_chunk.get_chunk_value(internal_chunk_cords);
        }
        else {
            return 0;    
        }
    }

    pub fn get_world_value_as_block(&self, cords : [i32 ; 3]) -> BlockTexture {
        BlockTexture::from_id(self.get_world_value(cords))
    }

    pub fn clear(&mut self) {
        println!("Clearing World");
        self.loaded_chunks.clear();
    }

    //=====================================
    // Tik
    //=====================================

    pub fn tik(&mut self, player_data: &PlayerData, event_manager: &mut EventManager) {   
        // Collect Debug Data
        let debug_data = event_manager.get_mut_debug_data();
        debug_data.clear_world_data();
        let chunks_loaded = format!("Chunks Loaded({})", self.loaded_chunks.len());
        debug_data.add_world_data(chunks_loaded);
        let chunks_to_gen = format!("Chunks to gen({})", self.chunks_to_generate.len());
        debug_data.add_world_data(chunks_to_gen);
        let chunks_to_load = format!("chunks to load({})", self.chunks_to_load.len());
        debug_data.add_world_data(chunks_to_load);


        // Generate Terrain
        let mut terrain_gen_per_tik = 1;
        while let Some(key) = self.chunks_to_generate.pop() {
            if let Some(chunk) = &mut self.loaded_chunks.get_mut(&key) {
                let area_to_gen = chunk.get_world_area();
                chunk.terrain_generated = true;
                player_data.world_gen.generate_area(self, area_to_gen);
                
                terrain_gen_per_tik-=1;
                if terrain_gen_per_tik <= 0{
                    break;
                }
            }
        }

        // Create ch
        while let Some(key) = self.chunks_to_load.pop() {
            if let Some(chunk) = &mut self.loaded_chunks.get_mut(&key) {
                chunk.loaded_this_tik = true;
            }
            else {
                self.create_chunk(key);
            }
        }

        let mut chunks_to_unload = Vec::new();
        for (key, chunk) in &mut self.loaded_chunks {
            if !chunk.loaded_this_tik {
                chunks_to_unload.push(*key);
            }
            else {
                chunk.loaded_this_tik = false;
            }
        }

        // More Debug Data
        let chunks_to_unload_debug = format!("Chunks to unload ({})", chunks_to_unload.len());
        debug_data.add_world_data(chunks_to_unload_debug);

        for k in chunks_to_unload {
            let chunk = self.loaded_chunks.remove(&k);
            if let Some(chunk) = chunk {
                event_manager.add_events(&chunk.free());
            }
        }
    }

    //=====================================
    // Event
    //=====================================

    pub fn load_chunk(
        &mut self, 
        cords : &[i16 ; 3]
    ) {
        let key = Self::chunk_cords_to_key(*cords);
        self.chunks_to_load.push(key);
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn render_world(
        &mut self, 
        texture_manager: &mut TextureManager, 
        tile_map_manager: &mut TileMapManager, 
    ) {
        let mut chunks: Vec<(&u64, &mut WorldChunk)> = self.loaded_chunks.iter_mut().collect();
        chunks.sort_by_key(|(_, chunk)| chunk.depth);

        for (_, chunk) in chunks {
            chunk.render(texture_manager, tile_map_manager);
        }
    }
}


