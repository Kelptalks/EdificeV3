use crate::game_data::screen::renderer::casted_block_manager::casted_block_manager::CastedChunkManager;

#[derive(Clone)]
pub struct WorldConfig {
    scale: i32,
    height_variation: u32,
}

impl WorldConfig {
    pub fn new() -> Self {
        WorldConfig {
            scale: 200,
            height_variation: 100,
        }
    }

    pub fn mod_scale(&mut self, scale: i32) {
        self.scale += scale;
    }
    pub fn get_scale(&self) -> i32 {
        self.scale
    }
    pub fn get_chunk_rendering_range(&self) -> u32 {
        return (self.get_scale() as u32 / CastedChunkManager::get_chunk_tile_scale() / 2) + 4;
    }

    pub fn get_height_variation(&self) -> u32 {
        self.height_variation
    }

}