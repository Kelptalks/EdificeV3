use std::collections::HashMap;

use crate::game_data::{TextureManager, World, chunk_manager::{chunk_manager::WorldChunkManager, loaded_chunk::CHUNK_SIZE_I32}, chunk_tile_map_manager::{chunk_render_data::ChunkRenderData, chunk_tile_set::ChunkTileSet}, player_data::{self, player_data::PlayerData}, screen::widget::world_rendering::area_rendering_manager::{ray_caster::casted_tile::CastedTile, raycast_thread_pool::RayCastingThreadPool}, tools::iso_cord_tool, types::BlockTexture};

pub struct ChunkTileSetManager {
    thread_pool: RayCastingThreadPool,

    chunk_tile_sets: HashMap<u64, ChunkTileSet>,

    tile_sets_to_cast: Vec<u64>,
    dirty_sets: Vec<u64>,
    /// Keys of tile sets that should be fully removed and their GPU meshes freed.
    sets_to_free: Vec<u64>,

    /// When true, each chunk's tile set is re-cast with Selector overlay blocks
    /// on all border faces so chunk boundaries are visible in-game.
    show_borders: bool,
}

impl ChunkTileSetManager {
    pub fn new() -> ChunkTileSetManager {
        ChunkTileSetManager {
            thread_pool: RayCastingThreadPool::new(),

            chunk_tile_sets: HashMap::new(),
            tile_sets_to_cast: Vec::new(),
            dirty_sets: Vec::new(),
            sets_to_free: Vec::new(),
            show_borders: false,
        }
    }

    //=====================================
    // Dirty / free queuing
    //=====================================

    pub fn queue_dirty_chunk(&mut self, key: u64) {
        if let Some(set) = self.chunk_tile_sets.get_mut(&key) {
            if !set.dirty {
                set.dirty = true;
                self.dirty_sets.push(key);
            }
        }
    }

    /// Schedule a tile set to be freed (mesh released, entry removed) on the next clean().
    pub fn queue_free_chunk(&mut self, key: u64) {
        self.sets_to_free.push(key);
    }

    //=====================================
    // Border debug overlay
    //=====================================

    /// Toggle Selector-block overlay on every chunk's border faces and force a full recast.
    pub fn toggle_borders(&mut self) {
        self.show_borders = !self.show_borders;
        let keys: Vec<u64> = self.chunk_tile_sets.keys().copied().collect();
        for key in keys {
            self.queue_dirty_chunk(key);
        }
    }

    //=====================================
    // Clean
    //=====================================

    pub fn clean(&mut self, chunk_manager: &WorldChunkManager, player_data: &PlayerData, texture_manager: &mut TextureManager) {
        // Free any tile sets scheduled for removal
        let to_free: Vec<u64> = self.sets_to_free.drain(..).collect();
        for key in to_free {
            if let Some(mut set) = self.chunk_tile_sets.remove(&key) {
                set.free(texture_manager);
            }
            self.tile_sets_to_cast.retain(|k| *k != key);
            self.dirty_sets.retain(|k| *k != key);
        }

        // Cancel in-flight tasks, assign border mods if enabled, then re-submit
        self.dirty_sets.retain(|set_key| {
            if let Some(set) = self.chunk_tile_sets.get_mut(set_key) {
                if set.dirty {
                    set.mark_dirty(texture_manager, &mut self.thread_pool);
                    
                    if let Some(chunk) = chunk_manager.get_loaded_chunk(set_key) {
                        set.set_lair_block_mods(chunk.collect_lair_block_mods(player_data));
                    }
                    

                }
                if let Some(chunk) = chunk_manager.get_loaded_chunk(set_key) {
                    set.ray_cast_set(&mut self.thread_pool, chunk);
                    set.dirty = false;
                    return false;
                }
            }
            true
        });

        // Submit new ray cast tasks for pending tile sets, throttled to avoid frame spikes
        const MAX_CASTS_PER_FRAME: usize = 8;
        let mut casts_submitted: usize = 0;
        self.tile_sets_to_cast.retain(|set_key| {
            if casts_submitted >= MAX_CASTS_PER_FRAME {
                return true;
            }
            if let Some(set) = self.chunk_tile_sets.get_mut(set_key) {
                if let Some(chunk) = chunk_manager.get_loaded_chunk(set_key) {
                    if self.show_borders {
                        let mods = set.get_area().generate_border_mods(BlockTexture::Selector);
                        set.set_lair_block_mods(mods);
                    }
                    set.ray_cast_set(&mut self.thread_pool, chunk);
                    casts_submitted += 1;
                    return false;
                }
            }
            true
        });

        // Pick up completed ray casts and bake meshes
        for tile_set in self.chunk_tile_sets.values_mut() {
            tile_set.clean(texture_manager, &mut self.thread_pool);
        }
    }

    //=====================================
    // Render
    //=====================================

    pub fn render(&mut self, texture_manager: &mut TextureManager, render_data: &ChunkRenderData) {
        let center = render_data.center_chunk_cords;
        let range = render_data.chunk_view_range;
        for z in -range[2]..=range[2] {
            for y in -range[1]..=range[1] {
                for x in -range[0]..=range[0] {
                    let chunk_key = World::chunk_cords_to_key([
                        center[0] + x,
                        center[1] + y,
                        center[2] + z,
                    ]);

                    if let Some(tile_set) = self.chunk_tile_sets.get_mut(&chunk_key) {
                        tile_set.render(texture_manager, render_data);
                    }
                }
            }
        }
    }

    //=====================================
    // Accessors
    //=====================================

    pub fn get_set(&self, key: &u64) -> Option<&ChunkTileSet> {
        self.chunk_tile_sets.get(key)
    }

    pub fn get_mut_set(&mut self, key: &u64) -> Option<&mut ChunkTileSet> {
        self.chunk_tile_sets.get_mut(key)
    }

    pub fn add_chunk_tile_set(&mut self, chunk_tile_set: ChunkTileSet, key: &u64) {
        self.chunk_tile_sets.insert(*key, chunk_tile_set);
        self.tile_sets_to_cast.push(*key);
    }

    //=====================================
    // Occlusion query
    //=====================================

    pub fn get_obscuring(&self, world_cords: [i32; 3], num_lairs: i16) -> Vec<&CastedTile> {
        let mut tiles = Vec::new();
        let flattened_cords = iso_cord_tool::flatten_world_cords(world_cords);
        let base_chunk_z = World::world_cords_to_chunk_cords(world_cords)[2];

        for i in 0..num_lairs {
            // Reconstruct the world position for this column at lair i to find the correct chunk.
            // All points on this isometric ray satisfy pos_x - pos_z = fx, pos_y - pos_z = fy,
            // so as wz rises, wx and wy rise equally — the tile lives in a chunk offset in all axes.
            let wz = (base_chunk_z as i32 + i as i32) * CHUNK_SIZE_I32;
            let wx = flattened_cords[0] + wz;
            let wy = flattened_cords[1] + wz;
            let base = World::world_cords_to_chunk_cords([wx, wy, wz]);

            // Within a single z-lair the column can cross into the next x-chunk, next y-chunk,
            // or both. Check all four candidates in ascending chunk-depth order so higher-depth
            // tiles paint on top.
            for dx in 0..=1i16 {
                for dy in 0..=1i16 {
                    let key = World::chunk_cords_to_key([base[0] + dx, base[1] + dy, base[2]]);
                    if let Some(tile_set) = self.get_set(&key) {
                        if let Some(tile) = tile_set.tile_map.get(&flattened_cords) {
                            tiles.push(tile);
                        }
                    }
                }
            }
        }
        tiles
    }

}
