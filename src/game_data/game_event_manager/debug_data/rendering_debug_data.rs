use crate::debug_fields;


pub struct RenderingDebugData {
    pub frame_time_ms: f32,
    
    pub tiles_rendered: u32,
    pub tiles_cashed: u32,

    pub chunks_cashed: u32,
    pub chunks_rendered: u32,

    pub free_cashed_textures: u32,

    pub entitys_drawn: u32,

    pub total_lairs: usize,

}

impl RenderingDebugData {
    pub fn new() -> RenderingDebugData {
        RenderingDebugData {
            frame_time_ms: 0.0,

            tiles_cashed: 0,
            tiles_rendered: 0,

            chunks_cashed: 0,
            chunks_rendered: 0,

            free_cashed_textures: 0,

            entitys_drawn: 0,

            total_lairs: 0,
        }
    }

    pub fn to_string_vec(&self) -> Vec<String> {
        debug_fields!(
            self, 
            tiles_rendered, 
            tiles_cashed, 
            chunks_rendered, 
            chunks_cashed, 
            entitys_drawn,
            total_lairs,
            frame_time_ms,
            free_cashed_textures
        )
    }

    
}