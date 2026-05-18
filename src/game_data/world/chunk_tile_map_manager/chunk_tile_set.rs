use std::collections::HashMap;

use crate::game_data::{TextureManager, World, chunk_manager::loaded_chunk::LoadedWorldChunk, chunk_tile_map_manager::chunk_render_data::ChunkRenderData, screen::{iso_cord_tool, widget::world_rendering::area_rendering_manager::{block_lair_manager::lair_block::LairBlockMod, ray_caster::casted_tile::CastedTile, raycast_thread_pool::{RayCastingTaskId, RayCastingThreadPool}}}, texture_manager::{texture::Texture, texture_cashe::texture_cashe::CashedTextureID}, world::locations::world_area::WorldArea};

pub struct ChunkTileSet {
    pub chunk_key: u64,
    area: WorldArea,

    tile_map: HashMap<[i32; 2], CastedTile>,

    ray_casting_task_id: Option<RayCastingTaskId>,

    cashed_texture_dirty: bool,
    cashed_texture_id: Option<CashedTextureID>,
}

impl ChunkTileSet {

    pub fn new(chunk: &LoadedWorldChunk) -> ChunkTileSet {
        let key = World::chunk_cords_to_key(chunk.get_cords());

        ChunkTileSet {
            chunk_key: key,
            area: chunk.get_world_area(),

            tile_map: HashMap::new(),

            ray_casting_task_id: None,

            cashed_texture_dirty: false,
            cashed_texture_id: None,
        }
    }

    pub fn ray_cast_set(&mut self, thread_pool: &mut RayCastingThreadPool, chunk: &LoadedWorldChunk) {
        let mut world_snapshot = World::new();
        world_snapshot.create_chunk(self.chunk_key);
        world_snapshot.set_chunk_block_data(self.chunk_key, chunk.clone_block_data());
        let lair_block_mods: Vec<LairBlockMod> = Vec::new();

        if self.ray_casting_task_id.is_none() {
            self.ray_casting_task_id = Some(thread_pool.submit(self.area, lair_block_mods, world_snapshot));
        }
    }

    //=====================================
    // Clean
    //=====================================

    pub fn clean(&mut self, texture_manager: &mut TextureManager, thread_pool: &mut RayCastingThreadPool) {
        // Pick up completed ray cast
        if let Some(id) = self.ray_casting_task_id {
            if let Some(map) = thread_pool.get_task_map(id) {
                self.tile_map = map;
                self.ray_casting_task_id = None;
                self.cashed_texture_dirty = true;
            }
        }

        // Bake tiles into cached texture
        if self.cashed_texture_dirty {
            let center_world = self.area.get_center_world_cords();
            let iso_center = iso_cord_tool::flatten_world_cords(center_world);
            let iso_center_f = [iso_center[0] as f32, iso_center[1] as f32];

            let dims = self.area.get_dimensions();
            let iso_extent = (dims[0] + dims[2]).max(dims[1] + dims[2]) as f32;
            let block_scale = 1.0 / iso_extent;

            if let Some(cashed_texture_id) = self.cashed_texture_id {
                texture_manager.get_mut_texture_cashe().clear_cashed_texture(cashed_texture_id);

                for (key, tile) in self.tile_map.iter() {
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

    //=====================================
    // Render
    //=====================================

    pub fn render_tiles(&mut self, texture_manager: &mut TextureManager, render_data: &ChunkRenderData) {
        for (_key, tile) in self.tile_map.iter_mut() {
            tile.render(texture_manager, render_data.scale, render_data.offset);
        }
    }

    pub fn render(&mut self, texture_manager: &mut TextureManager, render_data: &ChunkRenderData, LOD: usize) {
        if LOD > 0 {
            if let Some(cashed_texture_id) = self.cashed_texture_id {
                let center_world = self.area.get_center_world_cords();
                let iso_center = iso_cord_tool::flatten_world_cords(center_world);
                let iso_center_f = [iso_center[0] as f32, iso_center[1] as f32];

                let dims = self.area.get_dimensions();
                let iso_extent = (dims[0] + dims[2]).max(dims[1] + dims[2]) as f32;
                let half = iso_extent * render_data.scale;

                let mut center = iso_cord_tool::float_iso_to_ndc_cords(render_data.scale, iso_center_f);
                center[0] += render_data.offset[0];
                center[1] += render_data.offset[1];

                let pos = [
                    center[0] - half,
                    center[1] - half,
                    center[0] + half,
                    center[1] + half,
                ];

                texture_manager.render_texture(Texture::CashedTexture(cashed_texture_id), pos);
            }
        }
        else if !self.tile_map.is_empty() {
            self.render_tiles(texture_manager, render_data);
        }
    }
}
