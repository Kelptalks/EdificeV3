use std::sync::Arc;

use crate::game_data::screen::{iso_cord_tool, renderer::casted_block_manager::casted_chunk::CastedChunk};



#[derive(PartialEq, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Clone)]
pub struct CameraData {
    // window info
    window_rez : [f32; 2],
    viewport_rez : [f32; 2],
    viewport_offset : [f32; 2],

    cam_world_cords : [f32 ; 3],

    //Camera offset by controls
    ndc_render_offset : [f32; 2],

    // Render Location
    iso_cam_center_cords : [f32; 2],
    view_distance : usize,
    cashed_view_distance : usize,

    // Render Scaling
    zoom : f32,
    render_scale : f32,
    tile_pixel_scale : f32,
    tile_ndc_scale : f32,

    chunk_pixel_scale : f32,
    chunk_ndc_scale : f32,

    // Ray Casting
    direction : Direction,
    direction_mods : [i32; 3],

    // Frame Data
    frame_count: u32,
    frame_time: u32,

    // Performance caps
    draw_distance : usize,
    shadow_draw_distance: usize,
    max_chunk_cache_per_frame: u32,
}

impl CameraData {
    pub fn new() -> Self {
        Self {
            // Window info
            window_rez : [1920.0, 1080.0],
            viewport_rez : [0.0, 0.0],
            viewport_offset : [0.0, 0.0],

            // World cords
            cam_world_cords : [100.0, 100.0, 100.0],
            
            // Drawing Offsets
            ndc_render_offset : [0.0, 0.0],
            
            // Renderer location
            iso_cam_center_cords : [0.0, 0.0],
            view_distance : 4,
            cashed_view_distance : 30,

            // Render scaling
            zoom : 1.0,
            tile_ndc_scale : 0.0,
            render_scale : 0.0005,
            tile_pixel_scale : 32.0,

            chunk_pixel_scale : 32.0 * (CastedChunk::get_chunk_tile_dimensions() as f32),
            chunk_ndc_scale : 0.0,


            // Ray casting
            direction : Direction::North,
            direction_mods : [1, 1, 1],


            // Frame data
            frame_count: 0,
            frame_time: 0,
            
            // Performance caps
            max_chunk_cache_per_frame: 20,
            draw_distance : 200,
            shadow_draw_distance: 50,
        }
    }

    //=====================================
    // Updates
    //=====================================

    pub fn update_camera_values(&mut self) {
        self.tile_ndc_scale = self.get_render_scale() * (self.get_tile_pixel_scale() as f32);
        self.chunk_ndc_scale = self.get_render_scale() * (self.get_chunk_pixel_scale() as f32);

        // Get iso world cords of center of stream
        let screen_ndi_center = [
            -self.get_ndc_draw_offset()[0],
            -self.get_ndc_draw_offset()[1],
        ];

        let screen_center_iso_cords = iso_cord_tool::ndi_screen_cords_to_iso_cords(
            self.tile_ndc_scale,
            screen_ndi_center,
        );

        self.iso_cam_center_cords = [
            screen_center_iso_cords[0],
            screen_center_iso_cords[1],
        ];

    }

    //=====================================
    // Converters
    //=====================================

    pub fn world_to_casted_tile_cords(&self, world_cords: [i32; 3]) -> [i32; 2] {
        let z_mod = world_cords[2] - self.cam_world_cords[2] as i32;
        let x_cor = world_cords[0] - self.cam_world_cords[0] as i32 - z_mod + 1; // Wierd offset Don't know why it's needed
        let y_cor = world_cords[1] - self.cam_world_cords[1] as i32 - z_mod;
        let casted_tile_cords = [x_cor, y_cor];

        return casted_tile_cords;
    }

    pub fn casted_tile_cords_to_ndc_cords(&self, casted_tile_cords: [i32; 2]) -> [f32; 2] {
        let mut ndc_cords = iso_cord_tool::casted_to_ndc_cords(self.get_tile_ndc_scale(), casted_tile_cords);
        ndc_cords[0] += self.ndc_render_offset[0];
        ndc_cords[1] += self.ndc_render_offset[1];
        return ndc_cords;
    }

    pub fn world_to_ndc_cords(&self, world_cords: [i32; 3]) -> [f32; 2] {
        let casted_tile_cords = self.world_to_casted_tile_cords(world_cords);
        let ndc_cords = self.casted_tile_cords_to_ndc_cords(casted_tile_cords);
        return ndc_cords;
    }

    //=====================================
    // Viewport Data
    //=====================================

    pub fn set_viewport_rez(&mut self, rez: [f32; 2]) {
        self.viewport_rez = rez;
    }

    pub fn set_viewport_offset(&mut self, offset: [f32; 2]) {
        self.viewport_offset = offset;
    }

    pub fn get_viewport_rez(&self) -> [f32; 2] {
        return self.viewport_rez;
    }

    pub fn get_viewport_offset(&self) -> [f32; 2] {
        return self.viewport_offset;
    }

    //=====================================
    // ViewDistance 
    //=====================================

    pub fn get_view_distance(&self) -> usize {
        return self.view_distance;
    }

    pub fn get_cashed_view_distance(&self) -> usize {
        return self.cashed_view_distance;
    }


    //=====================================
    // Scaling
    //=====================================

    pub fn get_tile_ndc_scale(&self) -> f32 {
        return self.tile_ndc_scale;
    }

    pub fn get_tile_pixel_scale(&self) -> f32 {
        return self.tile_pixel_scale;
    }

    pub fn get_chunk_pixel_scale(&self) -> f32 {
        return self.chunk_pixel_scale;
    }

    pub fn get_chunk_ndc_scale(&self) -> f32 {
        return self.chunk_ndc_scale;
    }

    pub fn get_render_scale(&self) -> f32 {
        return self.render_scale * self.zoom;
    }

    //=====================================
    // Frame Data
    //=====================================

    pub fn increment_frame_number(&mut self) {
        self.frame_count += 1;
    }

    pub fn get_frame_count(&self) -> u32 {
        return self.frame_count;
    }

    pub fn set_frame_time(&mut self, frame_time: u32) {
        self.frame_time = frame_time;
    }
    pub fn get_frame_time(&self) -> u32 {
        return self.frame_time;
    }

    pub fn get_max_chunk_cache_per_frame(&self) -> u32 {
        return self.max_chunk_cache_per_frame;
    }

    //=====================================
    // Getters / Setters
    //=====================================

    pub fn get_arc_ref(self) -> Arc<CameraData> {
        return Arc::new(self);
    }

    pub fn get_window_rez(&self) -> [f32; 2] {
        return self.window_rez;
    }

    pub fn set_screen_rez(&mut self, new_rez: [f32; 2]) {
        self.window_rez = new_rez;
    }

    // Ray Casting getters
    pub fn get_draw_distance(&self) -> usize {
        return self.draw_distance;
    }
    pub fn get_shadow_draw_distance(&self) -> usize {
        return self.shadow_draw_distance;
    }

    pub fn get_direction_mods(&self) -> [i32; 3] {
        return self.direction_mods;
    }
    pub fn get_direction(&self) -> &Direction {
        return &self.direction
    }

    pub fn mod_x_cam_cor(&mut self, x_mod : f32) {
        self.ndc_render_offset[0] += x_mod;
    }
    pub fn mod_y_cam_cor(&mut self, y_mod : f32) {
        self.ndc_render_offset[1] += y_mod;
    }

    pub fn mod_scale(&mut self, zoom_mod: f32) {
        self.zoom *= zoom_mod;

        //self.render_scale *= zoom_mod;
    }

    pub fn get_ndc_draw_offset(&self) -> [f32 ; 2]{
        let x_draw_offset = self.ndc_render_offset[0];
        let y_draw_offset = self.ndc_render_offset[1];

        return [x_draw_offset, y_draw_offset];
    }

    pub fn get_cam_world_cords(&self) -> [f32 ; 3] {
        return self.cam_world_cords;
    }

    pub fn get_zoom(&self) -> f32 {
        return self.zoom;
    }

    pub fn get_iso_cam_center(&self) -> [f32; 2] {
        return self.iso_cam_center_cords;
    }

}
