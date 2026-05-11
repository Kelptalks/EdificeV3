use std::{cmp::max, collections::{HashMap, hash_map}};

use crate::game_data::{TextureManager, World, game_event_manager::{event_manager::{self, Event, EventManager}, render_event_manager::{render_event_manager::RenderEvent, texture_manager_event::TextureManagerEvent}}, locations::world_area::{self, WorldArea}, player_data::{self, player_data::PlayerData}, screen::{iso_cord_tool, renderer::casted_block_manager::casted_tile, text, widget::{prelude::play_world_view_config::PlayViewRenderingConfig, world_rendering::{area_rendering_manager::{self, area_rendering_manager::AreaRenderingManager, block_lair_manager::lair_block::{self, LairBlockMod}, ray_caster::casted_tile::CastedTile}, rendering_config, tile_map_manager}}}, texture_manager::{texture::Texture, texture_cashe::texture_cashe::CashedTextureID}, types::BlockTexture, world_chunk::{self, WorldChunk}};


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
    pub id: TileMapId,
    pub map: HashMap<[i32; 2], CastedTile>,

    min_depth: i32, // Used to convert world cords to map index
    max_depth: i32,
    
    min_key: [i32; 2],
    max_key: [i32; 2],

    pub ray_casting_dirty: bool,
    pub cashed_texture_dirty: bool,
    pub cashe_texture: bool,

    world_area: Option<WorldArea>,
    cashed_texture_id: Option<CashedTextureID>,
}



impl TileMap {
    pub fn new() -> TileMap {
        TileMap {
            id: TileMapId::get_next_id(),
            map: HashMap::new(),

            min_depth: 0,
            max_depth: 0,
            
            min_key: [0; 2],
            max_key: [0; 2],

            ray_casting_dirty: true,
            cashed_texture_dirty: true,
            cashe_texture: true,

            world_area: None,
            cashed_texture_id: None,
        }
    }
    
    pub fn set_world_area(&mut self, world_area: WorldArea) {
        self.world_area = Some(world_area);
        self.ray_casting_dirty = true;
        self.cashed_texture_dirty = true;
    }

    //=====================================
    // Getters / Setters
    //=====================================

    pub fn get_tile_with_tile_key(&self, iso_cords: &[i32; 2]) -> Option<&CastedTile> {
        self.map.get(iso_cords)
    }
    pub fn get_mut_tile_with_flattened_cords(&mut self, iso_cords: &[i32; 2]) -> Option<&mut CastedTile> {
        self.map.get_mut(iso_cords)
    }

    pub fn incert_tile_with_flattened_cords(&mut self, flattened_iso_cords: [i32; 2], tile: CastedTile) {
        // If the first incertion
        if self.map.is_empty() {
            self.min_key = flattened_iso_cords;
            self.max_key = flattened_iso_cords;
            
            if let Some(depth) = tile.get_triangles_depths().iter().max() {
                self.min_depth = *depth;
                self.max_depth = *depth;
            }
        }
        else {
            self.max_key[0] = self.max_key[0].max(flattened_iso_cords[0]);
            self.max_key[1] = self.max_key[1].max(flattened_iso_cords[1]);
            
            self.min_key[0] = self.min_key[0].min(flattened_iso_cords[0]);
            self.min_key[1] = self.min_key[1].min(flattened_iso_cords[1]);
        }
        self.map.insert(flattened_iso_cords, tile);
    }
    

    pub fn get_tile_at_area_cords(&self, world_cords: &[i32; 3]) -> Option<&CastedTile> {
        let flattened_iso_cords = iso_cord_tool::flatten_world_cords(*world_cords);
        self.get_tile_with_tile_key(&flattened_iso_cords)
    }

    pub fn incert_tile_with_area_cords(&mut self, world_cords: [i32; 3], tile: CastedTile) {
        let flattened_iso_cords = iso_cord_tool::flatten_world_cords(world_cords); 
        self.incert_tile_with_flattened_cords(flattened_iso_cords, tile);
    }

    pub fn get_map(&self) -> &HashMap<[i32; 2], CastedTile> {
        return &self.map
    }

    
    //=====================================
    // Rendering
    //=====================================


    pub fn ray_cast_world_area(&mut self, world: &World) {
        self.map.clear();
        if let Some(world_area) = self.world_area {
            
            let mut area_rendering_manager = AreaRenderingManager::new();
            area_rendering_manager.set_world_area(world_area);

            // Implement when chunks record game objects contained within them
            let lair_block_mods: Vec<LairBlockMod> = Vec::new();

            let tiles = area_rendering_manager.get_casted_tile_rays(world, &lair_block_mods);
            for tile in tiles {
                
                
                let world_cords = tile.get_world_cords();
                self.incert_tile_with_area_cords(world_cords, tile);
                
            }
            self.ray_casting_dirty = false;
        }
    }

    pub fn clean(&mut self, world: &World, texture_manager: &mut TextureManager) -> bool {
        if self.ray_casting_dirty {
            self.ray_cast_world_area(world);
        }
        if self.cashed_texture_dirty && self.cashe_texture {
            if let Some(world_area) = self.world_area {
                let center_world = world_area.get_center_world_cords();
                let iso_center = iso_cord_tool::flatten_world_cords(center_world);
                let iso_center_f = [iso_center[0] as f32, iso_center[1] as f32];

                let dims = world_area.get_dimensions();
                let iso_extent = (dims[0] + dims[2]).max(dims[1] + dims[2]) as f32;
                let block_scale = 1.0 / iso_extent;

                if let Some(cashed_texture_id) = self.cashed_texture_id {
                    for (key, tile) in self.map.iter() {
                        let offset_iso_cords = [
                            key[0] as f32 - iso_center_f[0],
                            key[1] as f32 - iso_center_f[1],
                        ];
                        let cords = iso_cord_tool::float_iso_to_ndc_cords(block_scale, offset_iso_cords);
                        tile.render_to_cashed_texture(texture_manager, cashed_texture_id, block_scale, cords);
                    }
                    self.cashed_texture_dirty = false;
                }
                else {
                    self.cashed_texture_id = texture_manager.get_free_cashed_texture();
                }
            }
        }

        if self.ray_casting_dirty {
            false
        }
        else if self.cashed_texture_dirty && self.cashe_texture {
            false
        }
        else {
            true
        }
    }

    pub fn render(
        &mut self,
        texture_manager: &mut TextureManager,
        draw_block_scale: f32,
        draw_offset: [f32; 2]
    ) {
        if let Some(cashed_texture_id) = self.cashed_texture_id {
            let texture = Texture::CashedTexture(cashed_texture_id);

            if let Some(world_area) = self.world_area {
                let center_world = world_area.get_center_world_cords();
                let iso_center = iso_cord_tool::flatten_world_cords(center_world);
                let iso_center_f = [iso_center[0] as f32, iso_center[1] as f32];

                let dims = world_area.get_dimensions();
                let iso_extent = (dims[0] + dims[2]).max(dims[1] + dims[2]) as f32;
                let half = iso_extent * draw_block_scale;

                let mut center = iso_cord_tool::float_iso_to_ndc_cords(draw_block_scale, iso_center_f);
                center[0] += draw_offset[0];
                center[1] += draw_offset[1];

                let pos = [
                    center[0] - half,
                    center[1] - half,
                    center[0] + half,
                    center[1] + half,
                ];

                texture_manager.render_texture(texture, pos);
            }
        }
        else {
            self.render_tiles(texture_manager, draw_block_scale, draw_offset);
        }
    }

    pub fn render_tiles(&mut self, texture_manager: &mut TextureManager, draw_block_scale: f32, draw_offset: [f32; 2]) {
        for (key, tile) in self.map.iter_mut() {
            tile.render(texture_manager, draw_block_scale, draw_offset);
        }
    }

    //=====================================
    // 
    //=====================================

    pub fn free(self) -> Vec<Event> {
        if let Some(id) = self.cashed_texture_id {
            vec![TextureManagerEvent::FreeCashedTexture(id).wrap_into_event()]
        }
        else {
            Vec::new()
        }
    }
    

}