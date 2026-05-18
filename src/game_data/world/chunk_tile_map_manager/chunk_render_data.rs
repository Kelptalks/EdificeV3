use crate::game_data::{chunk_manager::loaded_chunk::CHUNK_SIZE_I32, world::world::World};

pub struct ChunkRenderData {
    pub scale: f32,
    pub offset: [f32; 2],
    pub center_world_cords: [i32; 3],
    pub center_chunk_cords: [i16; 3],
    pub chunk_view_range: [i16; 3],
}

impl ChunkRenderData {
    pub fn new(scale: f32, offset: [f32; 2], center_world_cords: [i32; 3], view_range: f32) -> Self {
        let center_chunk_cords = World::world_cords_to_chunk_cords(center_world_cords);
        let r = ((view_range / CHUNK_SIZE_I32 as f32).ceil() as i16).max(1);
        Self {
            scale,
            offset,
            center_world_cords,
            center_chunk_cords,
            chunk_view_range: [r, r, r],
        }
    }
}
