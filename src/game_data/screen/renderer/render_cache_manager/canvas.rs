use miniquad::{GlContext, RenderingBackend, TextureFormat, TextureId, TextureParams};
use std::collections::HashMap;

use crate::game_data::{TextureManager, screen::{render_string, renderer::render_cache_manager::{canvas_data::CanvasData, canvas_chunk::CanvasChunk}}};

pub struct Canvas {
    texture_id: TextureId,

    // Tile management 
    canvas_map: HashMap<u64, CanvasChunk>,  // Bitpacked 2D coords -> CanvasChunk
    id_availability: Vec<bool>,             // Tracks which IDs are in use
    
    // Canvas properties
    canvas_data: CanvasData,

    max_tiles: u32,                         // tiles_per_row * tiles_per_row
    total_tiles: u32,
}

impl Canvas {
    pub fn new(ctx: &mut GlContext) -> Canvas {

        let canvas_data = CanvasData::new();
        let canvas_rez = canvas_data.canvas_rez as u32;

        let canvas_texture = ctx.new_render_texture(
            TextureParams {
                width: canvas_rez,
                height: canvas_rez,
                format: TextureFormat::RGBA8,
                ..TextureParams::default()
        }
        );

        Canvas{ 
            texture_id: canvas_texture,
            
            // Tile management 
            canvas_map: HashMap::new(),
            id_availability: vec![false; canvas_data.max_tiles as usize],
            
            // Canvas properties
            canvas_data: canvas_data,

            max_tiles: canvas_data.max_tiles,
            total_tiles: 0,
        }
    }



    // ===================
    //  Tile management
    // ===================

    /// Generate a bitpacked key from 2D isometric coordinates
    /// 
    /// Packs x and y into a single u64 (32 bits each)
    /// Properly handles negative coordinates by reinterpreting i32 as u32
    fn generate_key(iso_cords: [i32; 2]) -> u64 {
        let x = (iso_cords[0] as u32) as u64;
        let y = (iso_cords[1] as u32) as u64;
        (x << 32) | y
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
        if self.total_tiles >= self.max_tiles {
            println!("Canvas: Maximum tile limit reached.");
            return None;
        }

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
        let tile = CanvasChunk::new(iso_cords, canvas_id, self.canvas_data);
        self.canvas_map.insert(key, tile);

        self.total_tiles += 1;
        Some(canvas_id)
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

    // ===================
    //  Getters
    // ===================

    /// Get a tile by its isometric coordinates
    pub fn get_tile_at(&self, iso_cords: [i32; 2]) -> Option<&CanvasChunk> {
        let key = Self::generate_key(iso_cords);
        self.canvas_map.get(&key)
    }

     /// Get a tile by its isometric coordinates
    pub fn get_mut_tile_at(&mut self, iso_cords: [i32; 2]) -> Option<&mut CanvasChunk> {
        let key = Self::generate_key(iso_cords);
        self.canvas_map.get_mut(&key)
    }

    pub fn get_texture_id(&self) -> TextureId {
        self.texture_id
    }

    pub fn get_canvas_data(&self) -> &CanvasData {
        return &self.canvas_data;
    }

    // ===================
    // Rendering
    // ===================
    pub fn render_chunk_tiles(&mut self, texture_manager: &mut TextureManager, ctx: &mut GlContext) {
        for (_key, tile) in self.canvas_map.iter() {
            
            let mut canvas_ndc_cords = tile.canvas_ndc_cords;

        }
    }
}
