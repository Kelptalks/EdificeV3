use std::collections::HashMap;

use crate::game_data::{TextureManager, World, locations::world_area::WorldArea, player_data::{self, game_object::GameObjectType, player_data::PlayerData}, screen::{iso_cord_tool, widget::world_rendering::{area_rendering_manager::{area_rendering_manager::AreaRenderingManager, block_lair_manager::lair_block::LairBlockMod}, rendering_config, tile_map::{TileMap, TileMapId}, tile_map_manager::{self, TileMapManager}}}, texture_manager};

const CHUNK_SIZE: usize = 16;
const CHUNK_AREA: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub const CHUNK_SIZE_I32: i32 = 16;

pub struct WorldChunk {
    // World Data
    cords : [i16; 3],
    block_data : Box<[u16; CHUNK_VOLUME]>,

    // Cashed rendering
    dirty: bool,
    tile_map_id: Option<TileMapId>,

    // Game objects
    game_objects: Vec<GameObjectType>,
}

impl WorldChunk {
    pub fn new(chunk_cords :[i16; 3]) -> Self
    {
        Self {
            cords : chunk_cords,
            block_data : Box::new([0; CHUNK_VOLUME]),

            dirty: true,
            tile_map_id: None,

            game_objects: Vec::new(),
        }
    }

    //=====================================
    // Util
    //=====================================

    pub fn chunk_cords_to_world_cords(chunk_cords: [i16; 3]) -> [i32; 3] {
        [
            chunk_cords[0] as i32 * CHUNK_SIZE_I32,
            chunk_cords[1] as i32 * CHUNK_SIZE_I32,
            chunk_cords[2] as i32 * CHUNK_SIZE_I32,
        ]
    }

    pub fn get_chunk_size_i32() -> i32 {
        CHUNK_SIZE_I32
    }

    pub fn get_chunk_volume() -> usize {
        CHUNK_VOLUME
    }

    pub fn get_world_area(&self) -> WorldArea {
        let chunk_size = WorldChunk::get_chunk_size_i32();

        let min_x = self.cords[0] as i32 * chunk_size;
        let min_y = self.cords[1] as i32 * chunk_size;
        let min_z = self.cords[2] as i32 * chunk_size;

        let max_x = min_x + chunk_size - 1;
        let max_y = min_y + chunk_size - 1;
        let max_z = min_z + chunk_size - 1;

        let min_world_cords = [min_x, min_y, min_z];
        let max_world_cords = [max_x, max_y, max_z];

        WorldArea::new_with_cords([min_world_cords, max_world_cords])
    }

    pub fn get_depth(&self) -> i32 {
        let chunk_size = WorldChunk::get_chunk_size_i32();

        let min_x = self.cords[0] as i32 * chunk_size;
        let min_y = self.cords[1] as i32 * chunk_size;
        let min_z = self.cords[2] as i32 * chunk_size;

        iso_cord_tool::get_depth_from_world_cords([min_x, min_y, min_z])
    }

    //=====================================
    // Block Managment
    //=====================================

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

    pub fn get_cords(&self) -> [i16; 3] {
        return self.cords
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

    pub fn set_block_data(&mut self, block_data: Box<[u16; CHUNK_VOLUME]>) {
        self.block_data = block_data;
    }


    //=====================================
    // Rendering
    //=====================================

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn ray_cast_tile_map(&mut self, tile_map_manager: &mut TileMapManager) { 
        // Create a temp world for rendering
        // Why : this allows the chunk to be modified without passing in world
        // Also the only data required for rendering the chunk is in the chunk itself

        if let Some(id) = self.tile_map_id {
            if let Some(tile_map) = tile_map_manager.get_mut_tile_map(id) {
                let mut temp_world = World::new();
                temp_world.set_chunk_block_data(self.cords, self.block_data.clone());

                let world_area = self.get_world_area();
                

                tile_map.ray_cast_world_area(world_area, &temp_world);
                self.dirty = false;
            }
        }
        else {
            self.tile_map_id = Some(tile_map_manager.new_tile_map(self.get_depth()));
        }


    }

    //=====================================
    // Game Object Manamgnet
    //=====================================

    // Remove game objects that are not contained within the chunk
    pub fn update_game_objects(&mut self, player_data: &PlayerData) {
        
        for game_object in &mut self.game_objects {
            // let current_object = player_data.get_game_object();

        }
    }

}