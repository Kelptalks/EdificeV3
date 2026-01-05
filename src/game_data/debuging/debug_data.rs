use crate::game_data::{TextureManager, screen::{ScreenData, render_string, screen_data}, texture_manager};

pub struct DebugData {
    frame_time: u32,
    current_tik: u32,
    mouse_tile_cords: [i32; 2],
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
            frame_time: 0,
            current_tik: 0,
            mouse_tile_cords: [0, 0],
        }
    }

    //=====================================
    // Setters
    //=====================================

    pub fn set_frame_time(&mut self, frame_time: u32) {
        self.frame_time = frame_time;
    }

    pub fn set_current_tik(&mut self, current_tik: u32) {
        self.current_tik = current_tik;
    }

    pub fn set_mouse_tile_cords(&mut self, mouse_tile_cords: [i32; 2]) {
        self.mouse_tile_cords = mouse_tile_cords;
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn render_debug_data(&self, texture_manager: &mut TextureManager, screen_data: &ScreenData) {

        let formated_frame_time = format!("Frame Time: {} ms", self.frame_time);
        render_string(screen_data, texture_manager, formated_frame_time, "Basic".to_string(), 0.02, [0.0, 0.0]);

        let formated_tik_time = format!("Tik Time: {}", self.current_tik);
        render_string(screen_data, texture_manager, formated_tik_time, "Basic".to_string(), 0.02, [0.0, 30.0]);

        let mouse_tile_cords = format!("Mouse Tile Cords: ({:?})", self.mouse_tile_cords);
        render_string(screen_data, texture_manager, mouse_tile_cords, "Basic".to_string(), 0.02, [0.0, 60.0]);

    }
}