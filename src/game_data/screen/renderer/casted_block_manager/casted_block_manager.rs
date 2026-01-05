
use std::{collections::HashMap, sync::{Arc, RwLock}};

use image::imageops::tile;

use crate::game_data::{self, TextureManager, World, log_init, screen::{Camera, renderer::camera_data::CameraData}, texture_manager, types::{BlockTriangle, BlockType}};
use super::{casted_chunk::CastedChunk, casted_tile::CastedTile};
use super::super::{iso_cord_tool, ray_caster};



static CHUNK_TILE_DIMENSIONS : u32 = 16;
static CHUNK_TILE_AREA : u32 = CHUNK_TILE_DIMENSIONS * CHUNK_TILE_DIMENSIONS;

pub struct CastedChunkManager
{
    casted_chunk_map: HashMap<u32, Arc<RwLock<CastedChunk>>>,
    casted_chunk_raycasted_map: HashMap<u32, bool>,
    casted_chunk_key_list : Vec<u32>,
}

impl CastedChunkManager {

    pub fn new() -> Self {
        Self {
            casted_chunk_map: HashMap::new(),
            casted_chunk_key_list: Vec::new(),
            
            // Data for managing threads
            casted_chunk_raycasted_map: HashMap::new(),
        }
    }

    pub fn chunk_cords_to_key(cords : [i32; 2]) -> u32 {
        let x_bits = (cords[0] as u16) as u32;
        let y_bits = (cords[1] as u16) as u32;
        (x_bits << 16) | y_bits
    }

    pub fn create_chunk_at_cords(&mut self, camera_data : &CameraData, cords : [i32; 2])
    {
        let chunk_map_key = Self::chunk_cords_to_key(cords);
        let new_chunk = Arc::new(RwLock::new(CastedChunk::new(cords, camera_data)));
        self.casted_chunk_map.insert(chunk_map_key, new_chunk);
        self.casted_chunk_key_list.push(chunk_map_key);

        // Create raycasted map entry
        self.casted_chunk_raycasted_map.insert(chunk_map_key, false);
    }

    pub fn set_chunk_ray_casted_status(&mut self, cords : [i32; 2], status : bool)
    {
        let chunk_map_key = Self::chunk_cords_to_key(cords);
        if let Some(entry) = self.casted_chunk_raycasted_map.get_mut(&chunk_map_key) {
            *entry = status;
        }
    }

    pub fn get_chunk_ray_casted_status(&self, cords : [i32; 2]) -> bool
    {
        let chunk_map_key = Self::chunk_cords_to_key(cords);
        if let Some(status) = self.casted_chunk_raycasted_map.get(&chunk_map_key) {
            return *status;
        }
        return false;
    }

    pub fn get_chunk_at_chunk_cords(&self, cords : [i32; 2]) -> Option<Arc<RwLock<CastedChunk>>>
    {
        return self.casted_chunk_map.get(&Self::chunk_cords_to_key(cords)).map(Arc::clone);
    }

    pub fn get_chunk_at_tile_cords(&self, cords : [i32; 2]) -> Option<Arc<RwLock<CastedChunk>>>
    {
        let chunk_x_cor = cords[0] / CHUNK_TILE_DIMENSIONS as i32;
        let chunk_y_cor = cords[1] / CHUNK_TILE_DIMENSIONS as i32;

        self.get_chunk_at_chunk_cords([chunk_x_cor, chunk_y_cor])
    }


    pub fn get_chunk_cords_from_tile_cords(cords : [i32; 2]) -> [i32; 2] {
        let chunk_x_cor = cords[0] / CHUNK_TILE_DIMENSIONS as i32;
        let chunk_y_cor = cords[1] / CHUNK_TILE_DIMENSIONS as i32;

        return [chunk_x_cor, chunk_y_cor];
    }


    pub fn ray_cast_tile_at_casted_cords(&mut self, world: &Arc<RwLock<World>>, camera_data : &CameraData, cords: [i32; 2]) {
        let mut x_chunk_casted_cor = cords[0] / CHUNK_TILE_DIMENSIONS as i32;
        let mut y_chunk_casted_cor = cords[1] / CHUNK_TILE_DIMENSIONS as i32;

        let mut x_tile_internal_cor = cords[0] % CHUNK_TILE_DIMENSIONS as i32;
        let mut y_tile_internal_cor = cords[1] % CHUNK_TILE_DIMENSIONS as i32;

        if (x_tile_internal_cor < 0)
        {
            x_tile_internal_cor += CHUNK_TILE_DIMENSIONS as i32;
            x_chunk_casted_cor -= 1;
        }
        if (y_tile_internal_cor <  0)
        {
            y_tile_internal_cor += CHUNK_TILE_DIMENSIONS as i32;
            y_chunk_casted_cor -= 1;
        }

        let casted_chunk = self.get_chunk_at_chunk_cords([x_chunk_casted_cor, y_chunk_casted_cor]);
        let tile_index = (x_tile_internal_cor + (y_tile_internal_cor * CHUNK_TILE_DIMENSIONS as i32)) as usize;

        if let Some(casted_chunk) = casted_chunk {
            let mut guard = casted_chunk.write().unwrap();
            let world_guard = world.read().unwrap();
            ray_caster::raycast_tile_with_shadows(camera_data, &world_guard, guard.get_mut_tile_at_index(tile_index));
            
        }
        else {
            return;
        }
    }

    // Need to fix for arc_rwlock

    /*
    pub fn get_tile_at_casted_tile_cords(&mut self, cords : [i32; 2]) -> Option<&mut CastedTile> {
        
        let mut x_chunk_casted_cor = cords[0] / CHUNK_TILE_DIMENSIONS as i32;
        let mut y_chunk_casted_cor = cords[1] / CHUNK_TILE_DIMENSIONS as i32;

        let mut x_tile_internal_cor = cords[0] % CHUNK_TILE_DIMENSIONS as i32;
        let mut y_tile_internal_cor = cords[1] % CHUNK_TILE_DIMENSIONS as i32;

        if (x_tile_internal_cor < 0)
        {
            x_tile_internal_cor += CHUNK_TILE_DIMENSIONS as i32;
            x_chunk_casted_cor -= 1;
        }
        if (y_tile_internal_cor <  0)
        {
            y_tile_internal_cor += CHUNK_TILE_DIMENSIONS as i32;
            y_chunk_casted_cor -= 1;
        }

        println!("Chunk Internal Tile cords : ({}, {})", x_tile_internal_cor, y_tile_internal_cor);


        let casted_chunk = self.get_chunk_at_chunk_cords([x_chunk_casted_cor, y_chunk_casted_cor]);
        let tile_index = (x_tile_internal_cor + (y_tile_internal_cor * CHUNK_TILE_DIMENSIONS as i32)) as usize;


        if let Some(casted_chunk) = casted_chunk {
            return Some(casted_chunk.write().unwrap().get_mut_tile_at_index(tile_index).clone());
        }
        else {
            return None;
        }

        
    }

    
    /// Get a casted tile at the given isometric coordinates (converts f32 to i32)
    /// Returns None if the chunk containing the tile doesn't exist
    pub fn get_tile_at_iso_cords(&mut self, iso_cords: [f32; 2]) -> Option<&mut CastedTile> {
        // Convert float isometric coordinates to integer tile coordinates
        let tile_x = iso_cords[0] as i32;
        let tile_y = iso_cords[1] as i32;
        
        // Use the existing function to get the tile
        self.get_tile_at_casted_tile_cords([tile_x, tile_y])
    }

    */

}
