use crate::game_data::{TextureManager, screen::{ScreenData, text::render_string_at_ndc}, types::FontType};

pub struct DebugData {
    // Frame
    frame_count: u32,
    frame_time: u32,
    frame_high: u32,
    frame_high_frame_count: u32,

    // Tik data
    current_tik: u32,
    tiks_this_window: u32,
    tik_window_execution_time: u32,

    // Mouse data
    mouse_tile_cords: [i32; 2],

    // Rendering
    total_cached_chunks: u32,
    max_cached_chunks: u32, 
    total_tiles_raycasted: u32,

    // View Rendering
    direction: String,
    camera_cords: [i32; 3],

}
/*
################
## Debug Data ##
################
A struct that is passed down to collect data about parts of the program for displaying
*/



impl DebugData {
    pub fn new() -> DebugData {
        DebugData {
            // Frame
            frame_count: 0,
            frame_time: 0,
            frame_high: 0,
            frame_high_frame_count: 0,

            // Current tik
            current_tik: 0,
            tiks_this_window: 0,
            tik_window_execution_time: 0,

            // Mouse data
            mouse_tile_cords: [0, 0],
            
            // Rendering
            total_cached_chunks: 0,
            max_cached_chunks: 0, 
            total_tiles_raycasted: 0,

            // View Rendering
            direction: "North".to_string(),
            camera_cords: [0, 0, 0],
        }
    }

    //=====================================
    // Setters
    //=====================================

    // Frame Data
    pub fn set_frame_time(&mut self, frame_time: u32) {
        self.frame_time = frame_time;
    }
    pub fn set_frame_count(&mut self, frame_count: u32) {
        self.frame_count = frame_count;
    }


    // Tik Data
    pub fn set_tik_window_execution_time(&mut self, tik_execution_time: u32) {
        self.tik_window_execution_time = tik_execution_time;
    }
    pub fn set_tiks_this_window(&mut self, tiks: u32) {
        self.tiks_this_window = tiks;
    }
    pub fn set_current_tik(&mut self, current_tik: u32) {
        self.current_tik = current_tik;
    }

    // Mouse Data
    pub fn set_mouse_tile_cords(&mut self, mouse_tile_cords: [i32; 2]) {
        self.mouse_tile_cords = mouse_tile_cords;
    }


    // Rendering
    pub fn set_total_cached_chunks(&mut self, total_cached_chunks: u32) {
        self.total_cached_chunks = total_cached_chunks;
    }
    pub fn set_max_cached_chunks(&mut self, max_cached_chunks: u32) {
        self.max_cached_chunks = max_cached_chunks;
    }
    pub fn set_total_tiles_raycasted(&mut self, tiles_raycasted: u32) {
        self.total_tiles_raycasted = tiles_raycasted;
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn update_frame_data(&mut self) {
        // Reset frame high if it was 300 frames ago
        if self.frame_high_frame_count + 300 < self.frame_count {
            self.frame_high = 0;
        }

        // Check if frame time was highest ever
        if self.frame_high < self.frame_time {
            self.frame_high = self.frame_time;
            self.frame_high_frame_count = self.frame_count;
        }

    }

    pub fn render_debug_data(&mut self, texture_manager: &mut TextureManager, screen_data: &ScreenData) {
        // Update frame data 
        self.update_frame_data();
        
        // Don't render if not visible
        if !screen_data.get_debug_visiblity() {
            return;
        }

        // Text rendering data
        let screen_uv = screen_data.get_viewport_uv();      
        let mut current_text_render_ndi_cords = [screen_uv[0], screen_uv[1]];
        let scale = 0.01;
        let spacing = scale + 0.003;
        let font = FontType::Basic;

        // Render frame data
        let formated_frame_time = format!(
            "Frame count {} / Frame High: {} ms / Frame Time: {} ms / Total Tiles Raycasted: {}", 
            self.frame_count, 
            self.frame_high, 
            self.frame_time,
            self.total_tiles_raycasted
        );
        render_string_at_ndc(texture_manager, formated_frame_time, font, scale, current_text_render_ndi_cords);
        current_text_render_ndi_cords[1] += spacing;

        // Render tik time
        let formated_tik_time = format!(
            "Current Time: {} / Tik Window Execution Time: {} ms / Tiks in window: {}", 
            self.current_tik,
            self.tik_window_execution_time,
            self.tiks_this_window
        );
        render_string_at_ndc(texture_manager, formated_tik_time, font, scale, current_text_render_ndi_cords);
        current_text_render_ndi_cords[1] += spacing;

        // Render mouse location data
        let mouse_tile_cords = format!(
            "Mouse Tile Cords: ({:?})", 
            self.mouse_tile_cords
        );
        render_string_at_ndc(texture_manager, mouse_tile_cords, font, scale, current_text_render_ndi_cords);
        current_text_render_ndi_cords[1] += spacing;


        // Render rendering data
        let render_data = format!(
            "Cached Chunks: {}/{}", 
            self.total_cached_chunks,
            self.max_cached_chunks
        );
        render_string_at_ndc(texture_manager, render_data, font, scale, current_text_render_ndi_cords);
        current_text_render_ndi_cords[1] += spacing;

        // Render Camera Direction
        let direction_data = format!(
            "View_direction: {}", 
            self.direction,
        );
        render_string_at_ndc(texture_manager, direction_data, font, scale, current_text_render_ndi_cords);
        current_text_render_ndi_cords[1] += spacing;


        // Render Camera Direction
        let view_camera_cords = format!(
            "Ciew Camera Cords: {:?}", 
            self.camera_cords,
        );
        render_string_at_ndc(texture_manager, view_camera_cords, font, scale, current_text_render_ndi_cords);
        current_text_render_ndi_cords[1] += spacing;


    }
}