use std::collections::{HashMap, HashSet};
use std::time::Instant;
use crate::game_data::prof_record;

use crate::game_data::{TextureManager, World, chunk_manager::chunk_manager::WorldChunkType, game_event_manager::{event_manager::Event, game_event_manager::GameEventManager}, locations::world_area::WorldArea, player_data::{game_entity::{dynamic_entity_manager::dynamic_entity_manager::DynamicEntityId, game_entity_manager::{GameEntity, GameEntityId}}, player_data::PlayerData}, screen::{iso_cord_tool, widget::world_rendering::{tile_map::TileMapId, tile_map_manager::{TileMapEvent, TileMapManager}}}, world::world::WorldEvent};

const CHUNK_SIZE: usize = 16;
const CHUNK_AREA: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub const CHUNK_SIZE_I32: i32 = 16;

pub struct LoadedWorldChunk {
    pub time_till_unload: u64,

    // World Data
    cords : [i16; 3],
    block_data : Box<[u16; CHUNK_VOLUME]>,

    // Cashed rendering
    pub depth: i16,
    pub terrain_generated: bool,
    
    pub dirty: bool,
    pub tile_map_id: Option<TileMapId>,

    // Game objects
    pub block_entities: HashMap<[i32; 3], GameEntityId>,
    pub dynamic_entities: HashSet<DynamicEntityId>,
}

impl LoadedWorldChunk {
    pub fn wrap_into_chunk_type(self) -> WorldChunkType {
        WorldChunkType::Loaded(self)
    }
    
    pub fn new(chunk_cords :[i16; 3]) -> Self
    {
        let depth = chunk_cords[0] + chunk_cords[1] + chunk_cords[2];
        Self {
            time_till_unload: 1,

            cords : chunk_cords,
            block_data : Box::new([0; CHUNK_VOLUME]),

            terrain_generated: false,
            dirty: true,
            tile_map_id: None,
            depth,

            block_entities: HashMap::new(),
            dynamic_entities: HashSet::new(),
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

    pub fn get_world_cords(&self) -> [i32; 3] {
        [
            self.cords[0] as i32 * CHUNK_SIZE_I32,
            self.cords[1] as i32 * CHUNK_SIZE_I32,
            self.cords[2] as i32 * CHUNK_SIZE_I32,
        ]
    }

    pub fn get_chunk_size_i32() -> i32 {
        CHUNK_SIZE_I32
    }

    pub fn get_chunk_volume() -> usize {
        CHUNK_VOLUME
    }

    pub fn get_world_area(&self) -> WorldArea {
        let chunk_size = LoadedWorldChunk::get_chunk_size_i32();

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
        let chunk_size = LoadedWorldChunk::get_chunk_size_i32();

        let min_x = self.cords[0] as i32 * chunk_size;
        let min_y = self.cords[1] as i32 * chunk_size;
        let min_z = self.cords[2] as i32 * chunk_size;

        iso_cord_tool::get_depth_from_world_cords([min_x, min_y, min_z])
    }

    //=====================================
    // Block Managment
    //=====================================

    pub fn clone_block_data(&self) -> Box<[u16; CHUNK_VOLUME]> {
        self.block_data.clone()
    }

    pub fn overlaps_world_area(&self, world_area: &WorldArea) -> bool {
        let area_min = world_area.get_min_point().cords;
        let area_max = world_area.get_max_point().cords;
        let chunk_min = [
            self.cords[0] as i32 * CHUNK_SIZE_I32,
            self.cords[1] as i32 * CHUNK_SIZE_I32,
            self.cords[2] as i32 * CHUNK_SIZE_I32,
        ];
        let chunk_max = [
            chunk_min[0] + CHUNK_SIZE_I32 - 1,
            chunk_min[1] + CHUNK_SIZE_I32 - 1,
            chunk_min[2] + CHUNK_SIZE_I32 - 1,
        ];
        (0..3).all(|i| chunk_min[i] <= area_max[i] && chunk_max[i] >= area_min[i])
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

    pub fn get_cords(&self) -> [i16; 3] {
        return self.cords
    }

    pub fn get_key(&self) -> u64 {
        World::chunk_cords_to_key(self.cords)
    }

    pub fn fill(&mut self, value : u16) {
        self.block_data.fill(value);
    }

    // Set the value of at a cord in a chunk
    pub fn set_chunk_value(&mut self, value : u16, cords :[usize; 3]) {
        let cord_index = Self::cords_to_index(cords);
        self.block_data[cord_index] = value;
        self.dirty = true;
    }

    // Set the value of a cord in a chunk
    pub fn get_chunk_value(&self, cords :[usize; 3]) -> u16
    {
        let cord_index = Self::cords_to_index(cords);
        return self.block_data[cord_index];
    }

    pub fn set_block_data(&mut self, block_data: Box<[u16; CHUNK_VOLUME]>) {
        self.block_data = block_data;
        self.dirty = true;
    }


    //=====================================
    // Rendering
    //=====================================

    pub fn clean(&mut self, tile_map_manager: &mut TileMapManager) {
        if let Some(id) = self.tile_map_id {
            if let Some(tile_map) = tile_map_manager.get_mut_tile_map(id) {
                let world_area = self.get_world_area();
                tile_map.set_world_area(world_area);
            }
            tile_map_manager.dirty_id(&id);
            self.dirty = false;
        }
        else {
            self.tile_map_id = Some(tile_map_manager.new_tile_map());
        }
        
    }    

    pub fn render(
        &mut self, 
        player_data: &PlayerData,
        texture_manager: &mut TextureManager, 
        tile_map_manager: &mut TileMapManager
    ) {

        // Render cashed texture
        let block_scale = tile_map_manager.get_block_scale();
        let draw_offset = tile_map_manager.get_draw_offset();
        if self.dirty && self.terrain_generated {
            self.clean(tile_map_manager);
        }
        else if let Some(id) = self.tile_map_id {
            
            
            if let Some(tile_map) = tile_map_manager.get_mut_tile_map(id) {
                let t = Instant::now();
                tile_map.render(texture_manager, block_scale, draw_offset);
                prof_record("    chunk_tile_render", t.elapsed());

                // Render dynamic entities
                let t = Instant::now();
                for entity_id in &self.dynamic_entities {
                    if let Some(game_entity) = player_data.game_entity_manager.clone_game_entity(entity_id.wrap_into_game_entity_id()) {
                        if let GameEntity::DynamicEntity(dynamic_entity) = game_entity {
                            let texture = dynamic_entity.texture();
                            let pos = dynamic_entity.world_pos();
                            tile_map.render_enitity_at_world_pos(texture_manager, pos, texture, block_scale, draw_offset);
                        }
                    }
                }
                prof_record("    chunk_entity_render", t.elapsed());
            }


        }
        else {
            self.tile_map_id = Some(tile_map_manager.new_tile_map());
        }
        

        
    }

    //=====================================
    // Game Object Manamgnet
    //=====================================

    // Remove game objects that are not contained within the chunk
    pub fn update_game_entities(&mut self, _player_data: &PlayerData) {
        for _game_entity in &mut self.block_entities {

        }
    }

    pub fn free(self) -> Vec<Event> {
        if let Some(id) = self.tile_map_id {
            vec![TileMapEvent::FreeTileMap(id).wrap_into_event()]
        }
        else {
            Vec::new()
        }
    }
}



#[derive(Clone)]
pub enum WorldChunkEvent {

}

impl WorldChunkEvent {
    pub fn wrap_into_event(self, cords: [i16; 3]) -> Event {
        WorldEvent::LoadedChunkEvent(cords, self).wrap_into_event()
    }

    pub fn execute_chunk_event(&self, _loaded_chunk: &mut LoadedWorldChunk, _event_data: &mut GameEventManager) {



    }
}