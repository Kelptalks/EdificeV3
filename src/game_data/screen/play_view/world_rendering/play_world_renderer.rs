use std::sync::{Arc, RwLock};

use image::flat::View;

use crate::game_data::{TextureManager, World, debuging::debug_data::DebugData, screen::{camera_data::Direction, iso_cord_tool}, types::BlockType};


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



pub struct PlayWorldRender {
    // World
    zoom: i32,
    camera_cords: [i32; 3],
    view_direction: ViewDirection,

    // Rendering
    render_scale: f32,
}

impl PlayWorldRender {
    pub fn new() -> PlayWorldRender {
        PlayWorldRender {
            // World  
            zoom: 10,
            camera_cords: [0, 0, 0],
            view_direction: ViewDirection::North,


            // Rendering
            render_scale: 0.4,
        }
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn render_view(&mut self, texture_manager: &mut TextureManager, world: &Arc<RwLock<World>>) {
        let world = world.read().unwrap();

        let ndc_scale = self.render_scale / (self.zoom * 2) as f32;
        let ndc_x_draw_center_offset = ndc_scale;

        let direction_offsets = self.view_direction.offsets();

        // Loop through blocks in zoom
        for z in -self.zoom..=self.zoom {
            for y in -self.zoom..=self.zoom {
                for x in -self.zoom..=self.zoom {

                    let world_block_cords = [
                        self.camera_cords[0] + (x * direction_offsets[0]),
                        self.camera_cords[1] + (y * direction_offsets[1]),
                        self.camera_cords[2] + z,
                    ];

                    // Block Type
                    let block_type_at_cord = BlockType::from_id(world.get_world_value(world_block_cords));
                    
                    // Draw Cords
                    let mut draw_cords = iso_cord_tool::casted_to_ndc_cords(ndc_scale, [x - z, y - z]);
                    draw_cords[0] -= ndc_x_draw_center_offset;


                    texture_manager.render_block(block_type_at_cord, draw_cords, ndc_scale);
                }
            }
        }
    }


    //=====================================
    // Getters / Setters
    //=====================================

    // Add / Subtract cords 
    pub fn mod_cords(&mut self, cord_mods: [i32; 3]) {

        let direction_offsets = self.view_direction.offsets();

        for i in 0..self.camera_cords.len() {
            self.camera_cords[i] += (cord_mods[i] * direction_offsets[i]);
        }
    }

    // Mod amount of blocks in view
    pub fn mod_zoom(&mut self, zoom_mod: i32) {
        self.zoom += zoom_mod;
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
    // Debug
    //=====================================
    pub fn collect_debug_data(&self, debug_data: &mut DebugData) {
        debug_data.set_camera_cords(self.camera_cords);
        debug_data.set_direction(self.view_direction.to_string());
    }

}