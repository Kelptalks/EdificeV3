use miniquad::{GlContext, RenderingBackend, TextureId, TextureParams};

use crate::game_data::screen::renderer::render_cache_manager::canvas::canvas;



pub struct CanvasTile {
    pub iso_cords: [i32; 2],
    pub canvas_id: u32,
}

impl CanvasTile {
    pub fn new(iso_cords: [i32; 2], canvas_id: u32) -> CanvasTile {
        CanvasTile{ 
            iso_cords,
            canvas_id,
        }
    }
}
