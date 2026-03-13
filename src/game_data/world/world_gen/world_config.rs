use std::{cell::RefCell, rc::Rc};

use crate::game_data::{screen::renderer::casted_block_manager::casted_block_manager::CastedChunkManager};

#[derive(Clone)]
pub struct WorldConfig {
    scale: i32,
    height_variation: u32,

    // Links to toggles
    flat_world: Option<Rc<RefCell<bool>>>,
}

impl WorldConfig {
    pub fn new() -> Self {
        WorldConfig {
            scale: 75,
            height_variation: 100,

            flat_world: None,
        }
    }

    //=====================================
    // Link Management
    //=====================================
    pub fn set_flat_world_toggle_link(&mut self, link: Rc<RefCell<bool>>) {
        self.flat_world = Some(link);
    }

    pub fn world_flat(&self) -> bool {
        if let Some(link) = &self.flat_world {
            if *link.borrow() {
                return true;
            }
        }
        return false;
    }

    //=====================================
    // Direct Setters / Getters
    //=====================================
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