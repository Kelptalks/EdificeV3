use std::collections::HashMap;

use crate::game_data::{TextureManager, World, chunk_manager::chunk_manager::WorldChunkManager, chunk_tile_map_manager::{chunk_render_data::ChunkRenderData, chunk_tile_set::ChunkTileSet}, screen::widget::world_rendering::area_rendering_manager::raycast_thread_pool::RayCastingThreadPool};

pub struct ChunkTileSetManager {
    chunk_tile_sets: HashMap<u64, ChunkTileSet>,

    tile_sets_to_cast: Vec<u64>,
}

impl ChunkTileSetManager {
    pub fn new() -> ChunkTileSetManager {
        ChunkTileSetManager {
            chunk_tile_sets: HashMap::new(),
            tile_sets_to_cast: Vec::new(),
        }
    }

    pub fn clean(&mut self, chunk_manager: &WorldChunkManager, texture_manager: &mut TextureManager, thread_pool: &mut RayCastingThreadPool) {
        // Submit new ray cast tasks for pending tile sets
        self.tile_sets_to_cast.retain(|set_key| {
            if let Some(set) = self.chunk_tile_sets.get_mut(set_key) {
                if let Some(chunk) = chunk_manager.get_loaded_chunk(set_key) {
                    set.ray_cast_set(thread_pool, chunk);
                    return false;
                }
            }
            true
        });

        // Pick up completed ray casts and bake caches
        for tile_set in self.chunk_tile_sets.values_mut() {
            tile_set.clean(texture_manager, thread_pool);
        }
    }

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
                        if x.abs() + y.abs() + z.abs() < 2 {
                            tile_set.render(texture_manager, render_data, 0);
                        }
                        else {
                            tile_set.render(texture_manager, render_data, 1);
                        }
                        
                    }
                }
            }
        }
    }

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
}
