use std::collections::{HashMap, hash_map};

use crate::game_data::{TextureManager, World, locations::world_area::{self, WorldArea}, player_data::{self, player_data::PlayerData}, screen::{iso_cord_tool, renderer::casted_block_manager::casted_tile, widget::{prelude::play_world_view_config::PlayViewRenderingConfig, world_rendering::{area_rendering_manager::{self, area_rendering_manager::AreaRenderingManager, block_lair_manager::lair_block::{self, LairBlockMod}, ray_caster::casted_tile::CastedTile}, rendering_config}}}, world_chunk::{self, WorldChunk}};


use std::sync::atomic::{AtomicU32, Ordering};

static NEXT_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub struct TileMapId {
    id: u32,
}

impl TileMapId {
    pub fn get_next_id() -> TileMapId {
        TileMapId {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
        }
    }
}


pub struct TileMap {
    id: TileMapId,
    depth: i32, // Used to convert world cords to map index
    map: HashMap<[i32; 2], CastedTile>,

    is_cashed: bool,
    is_cashe_dirty: bool,
}



impl TileMap {
    pub fn new(depth: i32) -> TileMap {
        TileMap {
            id: TileMapId::get_next_id(),
            depth: depth,
            map: HashMap::new(),
            
            is_cashed: false,
            is_cashe_dirty: true,
        }
    }

    pub fn is_cashed(&self) -> bool{
        self.is_cashed
    }

    pub fn cashe(&mut self) {
        self.is_cashed = true;
    }

    pub fn is_cashe_dirty(&self) -> bool {
        self.is_cashe_dirty
    }

    //=====================================
    // Getters / Setters
    //=====================================

    pub fn get_id(&self) -> TileMapId {
        self.id
    }

    pub fn get_tile_map(&self) -> &HashMap<[i32; 2], CastedTile> {
        &self.map
    }

    pub fn get_tile_with_flattened_cords(&self, iso_cords: &[i32; 2]) -> Option<&CastedTile> {
        self.map.get(iso_cords)
    }
    pub fn get_mut_tile_with_flattened_cords(&mut self, iso_cords: &[i32; 2]) -> Option<&mut CastedTile> {
        self.map.get_mut(iso_cords)
    }


    pub fn incert_tile_with_flattened_cords(&mut self, flattened_iso_cords: [i32; 2], tile: CastedTile) {
        self.map.insert(flattened_iso_cords, tile);
    }
    

    pub fn get_tile_at_area_cords(&self, world_cords: &[i32; 3]) -> Option<&CastedTile> {
        let flattened_iso_cords = iso_cord_tool::flatten_world_cords(*world_cords);
        self.get_tile_with_flattened_cords(&flattened_iso_cords)
    }

    pub fn incert_tile_with_area_cords(&mut self, world_cords: [i32; 3], tile: CastedTile) {
        let flattened_iso_cords = iso_cord_tool::flatten_world_cords(world_cords); 
        self.incert_tile_with_flattened_cords(flattened_iso_cords, tile);
    }

    pub fn reset(&mut self, depth: i32) {
        self.depth = depth;
        self.map.clear();
    }

    pub fn get_map(&self) -> &HashMap<[i32; 2], CastedTile> {
        return &self.map
    }

    
    //=====================================
    // Rendering
    //=====================================


    pub fn ray_cast_world_area(&mut self, world_area: WorldArea, world: &World) {
        let mut area_rendering_manager = AreaRenderingManager::new();
        area_rendering_manager.set_world_area(world_area);

        // Implement when chunks record game objects contained within them
        let lair_block_mods: Vec<LairBlockMod> = Vec::new();

        let tiles = area_rendering_manager.get_casted_tile_rays(world, &lair_block_mods);
        for tile in tiles {
            if tile.struck() {
                let world_cords = tile.get_world_cords();
                self.incert_tile_with_area_cords(world_cords, tile);
            }
        }
    }

    pub fn render(&self, texture_manager: &mut TextureManager, draw_block_scale: f32, draw_offset: [f32; 2]) {
        for (key, tile) in self.map.iter() {
            tile.render(texture_manager, draw_block_scale, draw_offset);
        }
    }

    //=====================================
    // 
    //=====================================

    

}