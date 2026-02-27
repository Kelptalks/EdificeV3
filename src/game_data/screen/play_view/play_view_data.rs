use crate::game_data::{debuging::debug_data::DebugData, locations::world_area::WorldArea, types::BlockTexture};


/*
###############
## Direction ##
###############
Enum for controlling the cameras direction
main use is to get direction modify values
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum ViewDirection {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

impl ViewDirection {
    pub fn offsets(&self) -> [i32; 3] {
        match self {
            ViewDirection::North => [1, 1, 1],
            ViewDirection::South => [1, -1, 1],
            ViewDirection::East => [-1, -1, 1],
            ViewDirection::West => [-1, 1, 1],
        }
    }

    pub fn from_id(id: u8) -> Self {
        match id {
            0 => ViewDirection::North,
            1 => ViewDirection::South,
            2 => ViewDirection::East,
            3 => ViewDirection::West,
            _ => ViewDirection::North,
        }
    }

    pub fn id(&self) -> u8 {
        *self as u8
    }

    pub fn to_string(&self) -> String {
        match self {
            ViewDirection::North => "North".to_string(),
            ViewDirection::South => "South".to_string(),
            ViewDirection::East => "East".to_string(),
            ViewDirection::West => "West".to_string(),
        }
    }
}


/*
##########
## Mode ##
##########

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayMode {
    BuildManager, 
    DroneManager,
    LocationManager,
}

impl PlayMode {

}

/*
##################
## PlayViewData ##
##################
Manages information about the play view that needs to be shared
between UI elements, controls, and the renderer. 
*/
pub struct PlayViewData {
    // Controls
    play_mode: PlayMode,
    block_selected: BlockTexture,

    // UI rendering
    panel_padding_ndc_scale: f32,
    panel_tile_scale: f32,

    // Camera
    world_cords: [i32; 3],
    view_direction: ViewDirection,

    // World
    area_selected: WorldArea,
}

impl PlayViewData {
    pub fn new() -> PlayViewData {
        PlayViewData {
            // Controls
            play_mode: PlayMode::BuildManager,
            block_selected: BlockTexture::Air,

            // UI rendering
            panel_padding_ndc_scale: 0.025,
            panel_tile_scale: 0.01,

            // Camera
            world_cords: [0, 0, 0],
            view_direction: ViewDirection::North,

            // World
            area_selected: WorldArea::new_blank(),
        }
    }

    //=====================================
    // Controls
    //=====================================

    pub fn get_play_mode(&self) -> PlayMode {
        return self.play_mode;
    }

    pub fn set_play_mode(&mut self, play_mode: PlayMode) {
        self.play_mode = play_mode;
    }

    pub fn get_block_selected(&self) -> BlockTexture{
        return self.block_selected;
    }

    pub fn set_block_selected(&mut self, new_block: BlockTexture) {
        self.block_selected = new_block;
    }

    //=====================================
    // UI rendering
    //=====================================

    pub fn get_panel_padding_scale(&self) -> f32 {
        return self.panel_padding_ndc_scale;
    }

    pub fn get_panel_tile_scale(&self) -> f32 {
        return self.panel_tile_scale;
    }

    //=====================================
    // Camera 
    //=====================================

    // Add / Subtract cords 
    pub fn get_world_cords(&self) -> [i32; 3] {
        return self.world_cords;
    }

    pub fn get_view_direction_offsets(&self) -> [i32; 3] {
        return self.view_direction.offsets();
    }

    pub fn mod_world_cords(&mut self, cord_mods: [i32; 3]) {
        let direction_offsets = self.view_direction.offsets();
        for i in 0..self.world_cords.len() {
            self.world_cords[i] += cord_mods[i] * direction_offsets[i];
        }
    }

    pub fn set_world_cords(&mut self, cords: [i32; 3]) {
        self.world_cords = cords;
    }

    // Camera Direction
    pub fn rotate_left(&mut self) {
        let new_direction_id = self.view_direction.id() + 1;
        if new_direction_id < 4 {
            self.view_direction = ViewDirection::from_id(new_direction_id);
        }
        else {
            self.view_direction = ViewDirection::from_id(0);
        }
    }
    pub fn rotate_right(&mut self) {
        let current_direction_id = self.view_direction.id();

        if current_direction_id > 0 {
            self.view_direction = ViewDirection::from_id(current_direction_id - 1);
        }
        else {
            self.view_direction = ViewDirection::from_id(3);
        }
    }

    //=====================================
    // World
    //=====================================


    pub fn add_area(&mut self, area: WorldArea) {
        self.area_selected = area;
    }

    pub fn get_area_selected(&self) -> WorldArea {
        return self.area_selected;
    }

    //=====================================
    // Debug
    //=====================================

    pub fn collect_debug_data(&self, debug_data: &mut DebugData) {
        debug_data.set_camera_cords(self.world_cords);
        debug_data.set_direction(self.view_direction.to_string());
    }
}