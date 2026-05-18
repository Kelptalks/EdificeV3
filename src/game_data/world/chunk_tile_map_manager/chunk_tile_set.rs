use std::collections::HashMap;

use crate::game_data::{TextureManager, World, chunk_manager::loaded_chunk::LoadedWorldChunk, chunk_tile_map_manager::chunk_render_data::ChunkRenderData, screen::widget::world_rendering::area_rendering_manager::{block_lair_manager::lair_block::LairBlockMod, ray_caster::casted_tile::CastedTile, raycast_thread_pool::{RayCastingTaskId, RayCastingThreadPool}}, texture_manager::mesh_manager::texture_mesh::TextureMeshId, types::BlockTexture, world::locations::world_area::WorldArea};

pub struct ChunkTileSet {
    pub chunk_key: u64,
    area: WorldArea,

    pub tile_map: HashMap<[i32; 2], CastedTile>,

    ray_casting_task_id: Option<RayCastingTaskId>,

    mesh_id: Option<TextureMeshId>,

    pub dirty: bool,
}

impl ChunkTileSet {

    pub fn new(chunk: &LoadedWorldChunk) -> ChunkTileSet {
        let key = World::chunk_cords_to_key(chunk.get_cords());

        ChunkTileSet {
            chunk_key: key,
            area: chunk.get_world_area(),

            tile_map: HashMap::new(),

            ray_casting_task_id: None,

            mesh_id: None,

            dirty: false,
        }
    }

    pub fn mark_dirty(&mut self, texture_manager: &mut TextureManager, thread_pool: &mut RayCastingThreadPool) {
        if let Some(task_id) = self.ray_casting_task_id.take() {
            thread_pool.cancel_task(task_id);
        }
        self.tile_map.clear();
        self.free(texture_manager);
        self.dirty = true;
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
        if let Some(id) = self.ray_casting_task_id {
            if let Some(map) = thread_pool.get_task_map(id) {
                self.tile_map = map;
                self.ray_casting_task_id = None;
                self.build_mesh(texture_manager);
            }
        }
    }

    fn build_mesh(&mut self, texture_manager: &mut TextureManager) {
        let mesh_id = match self.mesh_id {
            Some(id) => {
                if let Some(mesh) = texture_manager.get_mut_mesh(id) {
                    mesh.clear();
                }
                id
            }
            None => {
                let id = texture_manager.new_mesh();
                let cords = World::key_to_chunk_cords(self.chunk_key);
                if let Some(mesh) = texture_manager.get_mut_mesh(id) {
                    mesh.depth = cords[0] as i32 + cords[1] as i32 + cords[2] as i32;
                }
                self.mesh_id = Some(id);
                id
            }
        };

        for (_key, tile) in &self.tile_map {
            tile.render_to_mesh(texture_manager, mesh_id);
        }
    }

    //=====================================
    // Render
    //=====================================

    pub fn render(&mut self, texture_manager: &mut TextureManager, render_data: &ChunkRenderData) {
        if let Some(mesh_id) = self.mesh_id {
            if let Some(mesh) = texture_manager.get_mut_mesh(mesh_id) {
                mesh.offset = render_data.offset;
                mesh.scale = render_data.scale;
            }
            texture_manager.render_mesh(mesh_id);
        }
    }

    pub fn free(&mut self, texture_manager: &mut TextureManager) {
        if let Some(id) = self.mesh_id.take() {
            texture_manager.remove_mesh(id);
        }
    }
}
