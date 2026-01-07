use crate::game_data::{TextureManager, screen::{ScreenData, text::render_string_at_ndi_cords}};

pub struct DebugData {
    // Frame
    frame_count: u32,
    frame_time: u32,
    frame_high: u32,
    frame_high_frame_count: u32,

    // Tik data
    current_tik: u32,
    tik_execution_time: u32,

    // Mouse data
    mouse_tile_cords: [i32; 2],

    // Rendering
    total_cached_chunks: u32,
    max_cached_chunks: u32, 
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
            tik_execution_time: 0,

            // Mouse data
            mouse_tile_cords: [0, 0],
            
            // Rendering
            total_cached_chunks: 0,
            max_cached_chunks: 0, 
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
    pub fn set_tik_execution_time(&mut self, tik_execution_time: u32) {
        self.tik_execution_time = tik_execution_time;
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
        let font = "Basic".to_string();

        // Render frame data
        let formated_frame_time = format!(
            "Frame count {} | Frame High: {} ms | Frame Time: {} ms", 
            self.frame_count, 
            self.frame_high, 
            self.frame_time
        );
        render_string_at_ndi_cords(texture_manager, formated_frame_time, font.clone(), scale, current_text_render_ndi_cords);
        current_text_render_ndi_cords[1] += spacing;

        // Render tik time
        let formated_tik_time = format!(
            "Current Time: {} | Tik Execution Time: {}", 
            self.current_tik,
            self.tik_execution_time
        );
        render_string_at_ndi_cords(texture_manager, formated_tik_time, font.clone(), scale, current_text_render_ndi_cords);
        current_text_render_ndi_cords[1] += spacing;

        // Render mouse location data
        let mouse_tile_cords = format!(
            "Mouse Tile Cords: ({:?})", 
            self.mouse_tile_cords
        );
        render_string_at_ndi_cords(texture_manager, mouse_tile_cords, font.clone(), scale, current_text_render_ndi_cords);
        current_text_render_ndi_cords[1] += spacing;


        // Render rendering data
        let render_data = format!(
            "Cached Chunks: {}/{}", 
            self.total_cached_chunks,
            self.max_cached_chunks
        );
        render_string_at_ndi_cords(texture_manager, render_data, font.clone(), scale, current_text_render_ndi_cords);
        current_text_render_ndi_cords[1] += spacing;

    }
}