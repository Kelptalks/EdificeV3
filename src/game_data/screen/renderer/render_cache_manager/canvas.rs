use miniquad::{GlContext, RenderingBackend, TextureId, TextureParams};
use std::collections::HashMap;

use crate::game_data::screen::renderer::render_cache_manager::canvas_tile::CanvasTile;

pub struct canvas {
    texture_id: TextureId,

    // Tile management 
    canvas_map: HashMap<u64, CanvasTile>,  // Bitpacked 2D coords -> CanvasTile
    id_availability: Vec<bool>,             // Tracks which IDs are in use
    canvas_tile_scale: u32,
    tiles_per_row: u32,
    max_tiles: u32,                         // tiles_per_row * tiles_per_row
}

impl canvas {
    fn new(ctx: &mut GlContext) -> canvas {
        let canvas_tile_scale = 1024;
        let tiles_per_row = 8;
        let max_tiles = tiles_per_row * tiles_per_row;

        let canvas_rez = canvas_tile_scale * tiles_per_row;
        
        let canvas_texture = ctx.new_render_texture(
            TextureParams {
                width: canvas_rez,
                height: canvas_rez,
                ..TextureParams::default()
        }
        );
        
        canvas{ 
            texture_id: canvas_texture,
            canvas_map: HashMap::new(),
            id_availability: vec![false; max_tiles as usize],
            canvas_tile_scale,
            tiles_per_row,
            max_tiles,
        }
    }

    /// Generate a bitpacked key from 2D isometric coordinates
    /// Packs x and y into a single u64 (32 bits each)
    fn generate_key(iso_cords: [i32; 2]) -> u64 {
        let x = iso_cords[0] as u64;
        let y = iso_cords[1] as u64;
        (x << 32) | (y & 0xFFFFFFFF)
    }

    /// Find the next available ID
    fn find_free_id(&self) -> Option<u32> {
        self.id_availability.iter()
            .position(|&used| !used)
            .map(|pos| pos as u32)
    }

    /// Create a new tile at the given isometric coordinates
    /// Returns the canvas_id if successful, or None if no IDs are available
    pub fn create_tile_at(&mut self, iso_cords: [i32; 2]) -> Option<u32> {
        // Check if tile already exists at these coordinates
        let key = Self::generate_key(iso_cords);
        if self.canvas_map.contains_key(&key) {
            return None;
        }

        // Find a free ID
        let canvas_id = self.find_free_id()?;

        // Mark ID as used
        self.id_availability[canvas_id as usize] = true;

        // Create and store the tile
        let tile = CanvasTile::new(iso_cords, canvas_id);
        self.canvas_map.insert(key, tile);

        Some(canvas_id)
    }

    /// Calculate UV coordinates for a tile based on its ID
    /// Returns [u_min, v_min, u_max, v_max]
    pub fn calculate_uv_for_id(&self, canvas_id: u32) -> [f32; 4] {
        if canvas_id >= self.max_tiles {
            // Return default if ID is out of bounds
            return [0.0, 0.0, 0.0, 0.0];
        }

        // Calculate tile position in grid
        let tile_x = canvas_id % self.tiles_per_row;
        let tile_y = canvas_id / self.tiles_per_row;

        // Calculate normalized UV coordinates
        let tile_size = 1.0 / self.tiles_per_row as f32;
        let u_min = tile_x as f32 * tile_size;
        let v_min = tile_y as f32 * tile_size;
        let u_max = u_min + tile_size;
        let v_max = v_min + tile_size;

        [u_min, v_min, u_max, v_max]
    }

    /// Free an ID and remove the tile at given coordinates
    pub fn free_tile_at(&mut self, iso_cords: [i32; 2]) -> bool {
        let key = Self::generate_key(iso_cords);
        
        if let Some(tile) = self.canvas_map.remove(&key) {
            // Mark ID as available
            self.id_availability[tile.canvas_id as usize] = false;
            true
        } else {
            false
        }
    }

    /// Free an ID directly
    pub fn free_id(&mut self, canvas_id: u32) -> bool {
        if canvas_id >= self.max_tiles {
            return false;
        }

        if self.id_availability[canvas_id as usize] {
            self.id_availability[canvas_id as usize] = false;
            
            // Find and remove the tile with this ID
            let key_to_remove = self.canvas_map.iter()
                .find(|(_, tile)| tile.canvas_id == canvas_id)
                .map(|(key, _)| *key);
            
            if let Some(key) = key_to_remove {
                self.canvas_map.remove(&key);
            }
            
            true
        } else {
            false
        }
    }

    /// Get a tile by its isometric coordinates
    pub fn get_tile_at(&self, iso_cords: [i32; 2]) -> Option<&CanvasTile> {
        let key = Self::generate_key(iso_cords);
        self.canvas_map.get(&key)
    }


}
