use std::collections::HashMap;

use crate::game_data::{TextureManager, World, locations::world_area::{self, WorldArea}, player_data::{self, player_data::PlayerData}, screen::{iso_cord_tool, renderer::casted_block_manager::casted_tile, widget::{prelude::play_world_view_config::PlayViewRenderingConfig, world_rendering::{area_rendering_manager::{self, area_rendering_manager::AreaRenderingManager, block_lair_manager::lair_block::{self, LairBlockMod}, ray_caster::casted_tile::CastedTile}, rendering_config}}}, world_chunk::{self, WorldChunk}};

pub struct TileMap {
    map_root_world_cords: [i32; 3], // Used to convert world cords to map index
    map: HashMap<[i32; 2], CastedTile>,
}



impl TileMap {
    pub fn new(root_world_cords: [i32; 3]) -> TileMap {
        TileMap {
            map_root_world_cords: root_world_cords,
            map: HashMap::new(),
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================

    pub fn get_tile_with_flattened_cords(&self, iso_cords: &[i32; 2]) -> Option<&CastedTile> {
        self.map.get(iso_cords)
    }


    pub fn incert_tile_with_flattened_cords(&mut self, flattened_iso_cords: [i32; 2], tile: CastedTile) {
        self.map.insert(flattened_iso_cords, tile);
    }
    

    pub fn get_tile_at_world_cords(&self, world_cords: &[i32; 3]) -> Option<&CastedTile> {
        let flattened_iso_cords = iso_cord_tool::flatten_world_cords(*world_cords);
        self.get_tile_with_flattened_cords(&flattened_iso_cords)
    }

    pub fn incert_tile_with_world_cords(&mut self, world_cords: [i32; 3], tile: CastedTile) {
        let flattened_iso_cords = iso_cord_tool::flatten_world_cords(world_cords); 
        self.incert_tile_with_flattened_cords(flattened_iso_cords, tile);
    }

    pub fn reset(&mut self, root_world_cords: [i32; 3]) {
        self.map_root_world_cords = root_world_cords;

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
            let world_cords = tile.get_world_cords();
            self.incert_tile_with_world_cords(world_cords, tile);
        }
    }

    pub fn render(&self, texture_manager: &mut TextureManager, draw_block_scale: f32, draw_offset: [f32; 2]) {
        for (key, tile) in self.map.iter() {
            tile.render(texture_manager, draw_block_scale, draw_offset);
        }
    } 

}