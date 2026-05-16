use std::{cmp::max, collections::{HashMap, hash_map}, time::Instant};
use crate::game_data::prof_record;

use crate::game_data::{TextureManager, World, game_event_manager::{event_manager::Event, render_event_manager::texture_manager_event::TextureManagerEvent}, locations::world_area::WorldArea, screen::{iso_cord_tool, widget::world_rendering::area_rendering_manager::{area_rendering_manager::AreaRenderingManager, block_lair_manager::lair_block::LairBlockMod, ray_caster::casted_tile::CastedTile, raycast_thread_pool::{RayCastingTaskId, RayCastingThreadPool}}}, texture_manager::{texture::Texture, texture_cashe::texture_cashe::CashedTextureID}};


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

    pub depth: i32, // Used to convert world cords to map index
    
    min_key: [i32; 2],
    max_key: [i32; 2],

    pub ray_casting_dirty: bool,
    pub ray_casting_task_id: Option<RayCastingTaskId>,

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

            depth: 0,
            
            min_key: [0; 2],
            max_key: [0; 2],

            ray_casting_dirty: true,
            ray_casting_task_id: None,

            cashed_texture_dirty: true,
            cashe_texture: true,

            world_area: None,
            cashed_texture_id: None,
        }
    }
    
    pub fn set_world_area(&mut self, world_area: WorldArea) {
        self.world_area = Some(world_area);
        self.depth = iso_cord_tool::get_depth_from_world_cords(world_area.get_center_world_cords());
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


    pub fn start_ray_casting(&mut self, world: &World, thread_pool: &mut RayCastingThreadPool) {
        if let Some(world_area) = self.world_area {
            let lair_block_mods: Vec<LairBlockMod> = Vec::new();
            let world_snapshot = world.world_snapshot(world_area);
            self.ray_casting_task_id = Some(thread_pool.submit(world_area, lair_block_mods, world_snapshot));
        }
    }

    pub fn is_clean(&self) -> bool {
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

    pub fn clean(&mut self, world: &World, texture_manager: &mut TextureManager, thread_pool: &mut RayCastingThreadPool) -> bool {
        if self.ray_casting_dirty {
            if let Some(task_id) = self.ray_casting_task_id {
                if let Some(map) = thread_pool.get_task_map(task_id) {
                    self.map = map;
                    self.ray_casting_dirty = false;
                    self.ray_casting_task_id = None;
                }
            }
            else {
                self.start_ray_casting(world, thread_pool);
            }
            


        }
        if !self.ray_casting_dirty && self.cashed_texture_dirty && self.cashe_texture {
            if let Some(world_area) = self.world_area {
                let center_world = world_area.get_center_world_cords();
                let iso_center = iso_cord_tool::flatten_world_cords(center_world);
                let iso_center_f = [iso_center[0] as f32, iso_center[1] as f32];

                let dims = world_area.get_dimensions();
                let iso_extent = (dims[0] + dims[2]).max(dims[1] + dims[2]) as f32;
                let block_scale = 1.0 / iso_extent;

                if let Some(cashed_texture_id) = self.cashed_texture_id {
                    texture_manager.get_mut_texture_cashe().clear_cashed_texture(cashed_texture_id);

                    let t = Instant::now();
                    for (key, tile) in self.map.iter() {
                        let offset_iso_cords = [
                            key[0] as f32 - iso_center_f[0],
                            key[1] as f32 - iso_center_f[1],
                        ];
                        let cords = iso_cord_tool::float_iso_to_ndc_cords(block_scale, offset_iso_cords);
                        tile.render_to_cashed_texture(texture_manager, cashed_texture_id, block_scale, cords);
                    }
                    prof_record("    cache_bake", t.elapsed());

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

                let t = Instant::now();
                texture_manager.render_texture(texture, pos);
                prof_record("    render_from_cache", t.elapsed());
            }
        }
        else {
            self.render_tiles(texture_manager, draw_block_scale, draw_offset);
        }
    }

    pub fn render_enitity_at_world_pos(
        &mut self, 
        texture_manager: &mut TextureManager, 
        world_pos: [f32; 3], 
        texture: Texture,

        draw_block_scale: f32,
        draw_offset: [f32; 2]
    ) {

        let mut draw_cords = iso_cord_tool::world_pos_to_ndc_cords(draw_block_scale, world_pos);

        draw_cords[0] += draw_offset[0];
        draw_cords[1] += draw_offset[1];

        // Scale to size of block
        let draw_pos = [
            draw_cords[0] - draw_block_scale, 
            draw_cords[1] - draw_block_scale,
            draw_cords[0] + draw_block_scale, 
            draw_cords[1] + draw_block_scale,
        ];
        
        texture_manager.render_texture(texture, draw_pos);

        let sprite_depth = iso_cord_tool::get_depth_from_world_cords(iso_cord_tool::world_pos_to_world_cords(world_pos));
        let flattened_cords = iso_cord_tool::world_pos_to_tile_cords(world_pos);

        for x in -3..=3 {
            for y in -3..=3 {
                let cords = [
                    flattened_cords[0] + x,
                    flattened_cords[1] + y,
                ];
                let tile = self.get_tile_with_tile_key(&cords);

                if let Some(tile) = tile {
                    let left_tile_world_cords = tile.get_left_triangle().get_solid_block_struck_cords();
                    let left_tile_depth = iso_cord_tool::get_depth_from_world_cords(left_tile_world_cords);
                    let re_render_left = left_tile_depth > sprite_depth;


                    let right_tile_world_cords = tile.get_right_triangle().get_solid_block_struck_cords();
                    let right_tile_depth = iso_cord_tool::get_depth_from_world_cords(right_tile_world_cords);
                    let re_render_right = right_tile_depth > sprite_depth;


                    if re_render_left {
                        tile.render_left_triangle(
                            texture_manager,
                            draw_block_scale,
                            draw_offset
                        );
                    }
                    if re_render_right {
                        tile.render_right_triangle(
                            texture_manager,
                            draw_block_scale,
                            draw_offset
                        );
                    }
                }
            }
        }
    }

    pub fn render_tiles(&mut self, texture_manager: &mut TextureManager, draw_block_scale: f32, draw_offset: [f32; 2]) {
        let t = Instant::now();
        for (key, tile) in self.map.iter_mut() {
            tile.render(texture_manager, draw_block_scale, draw_offset);
        }
        prof_record("    render_tiles_direct", t.elapsed());
    }

    //=====================================
    // 
    //=====================================

    pub fn free(self, thread_pool: &mut RayCastingThreadPool) -> Vec<Event> {
        if let Some(task_id) = self.ray_casting_task_id {
            thread_pool.cancel_task(task_id);
        }
        if let Some(id) = self.cashed_texture_id {
            vec![TextureManagerEvent::FreeCashedTexture(id).wrap_into_event()]
        }
        else {
            Vec::new()
        }
    }
    

}