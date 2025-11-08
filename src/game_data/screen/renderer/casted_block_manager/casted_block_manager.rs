
use std::collections::HashMap;

use image::imageops::tile;

use crate::game_data::{log_init, texture_manager, screen::Camera, TextureManager, Types::{BlockTriangle, BlockType}, World};
use super::{casted_chunk::CastedChunk, casted_tile::CastedTile};
use super::super::{iso_cord_tool, ray_caster, CameraData};



static CHUNK_TILE_DIMENSIONS : u32 = 16;
static CHUNK_TILE_AREA : u32 = CHUNK_TILE_DIMENSIONS * CHUNK_TILE_DIMENSIONS;

pub struct CastedChunkManager
{
    casted_chunk_map : HashMap<u32, CastedChunk>,
    casted_chunk_key_list : Vec<u32>,
}

impl CastedChunkManager {

    pub fn new() -> Self {
        Self {
            casted_chunk_map: HashMap::new(),
            casted_chunk_key_list: Vec::new(),
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
        self.casted_chunk_map.insert(chunk_map_key, CastedChunk::new(cords, camera_data));
        self.casted_chunk_key_list.push(chunk_map_key);
    }

    pub fn get_chunk_at_chunk_cords(&self, cords : [i32; 2]) -> Option<&CastedChunk>
    {
        return self.casted_chunk_map.get(&Self::chunk_cords_to_key(cords));
    }

    pub fn get_mut_chunk_at_chunk_cords(&mut self, cords : [i32; 2]) -> Option<&mut CastedChunk>
    {
        return self.casted_chunk_map.get_mut(&Self::chunk_cords_to_key(cords));
    }

    pub fn get_chunk_at_tile_cords(&self, cords : [i32; 2]) -> Option<&CastedChunk>
    {
        let chunk_x_cor = cords[0] / CHUNK_TILE_DIMENSIONS as i32;
        let chunk_y_cor = cords[1] / CHUNK_TILE_DIMENSIONS as i32;

        return self.get_chunk_at_chunk_cords([chunk_x_cor, chunk_y_cor]);
    }

    pub fn get_mut_chunk_at_tile_cords(&mut self, cords : [i32; 2]) -> Option<&mut CastedChunk>
    {
        let chunk_x_cor = cords[0] / CHUNK_TILE_DIMENSIONS as i32;
        let chunk_y_cor = cords[1] / CHUNK_TILE_DIMENSIONS as i32;

        return self.get_mut_chunk_at_chunk_cords([chunk_x_cor, chunk_y_cor])
    }


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


        let casted_chunk = self.get_mut_chunk_at_chunk_cords([x_chunk_casted_cor, y_chunk_casted_cor]);
        let tile_index = (x_tile_internal_cor + (y_tile_internal_cor * CHUNK_TILE_DIMENSIONS as i32)) as usize;


        if let Some(casted_chunk) = casted_chunk {
            return Some(casted_chunk.get_mut_tile_at_index(tile_index));
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

    pub fn render_all_chunks(&mut self, camera_data : &CameraData, texture_manager : &mut TextureManager, world : &World)
    {
        for chunk_key in &mut self.casted_chunk_key_list {
            let casted_chunk = self.casted_chunk_map.get_mut(chunk_key).unwrap();
            casted_chunk.render_chunk(camera_data, texture_manager, world);
        }
    }

}
