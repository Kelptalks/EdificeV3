use std::sync::{Arc, RwLock};

use image::flat::View;
use miniquad::{KeyCode, MouseButton};

use crate::game_data::{TextureManager, World, debuging::debug_data::DebugData, game_event_manager::{game_event_manager::{self, GameEventManager}, world_event_manager::world_event_manager::WorldEvent}, screen::{ScreenData, camera_data::Direction, iso_cord_tool, screen_data, text}, types::BlockType};

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



static MAX_VIEW_DISTANCE: i32 = 20;
static MIN_VIEW_DISTANCE: i32 = 1;

pub struct PlayWorldRender {
    // World
    zoom: i32,
    camera_cords: [i32; 3],
    camera_ndc_offset: [f32; 2],
    view_direction: ViewDirection,
    render_scale: f32,
    x_drawing_offset: f32,

    // Cached scale values
    ndc_block_scale: f32,
    ndc_tile_scale: f32,
    ndc_tile_half_scale: f32,
}

impl PlayWorldRender {
    pub fn new() -> PlayWorldRender {
        PlayWorldRender {



            // Rendering
            zoom: 10,
            camera_cords: [0, 0, 0],
            camera_ndc_offset: [0.0, 0.0],
            view_direction: ViewDirection::North,

            render_scale: 0.4,

            x_drawing_offset: 0.1,

            ndc_block_scale: 0.0,
            ndc_tile_scale: 0.0,
            ndc_tile_half_scale: 0.0,
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
        let new_zoom = self.zoom + zoom_mod;
        if new_zoom > MIN_VIEW_DISTANCE && new_zoom < MAX_VIEW_DISTANCE {
            self.zoom = new_zoom;
        }
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

    pub fn set_x_offset(&mut self, offset: f32) {
        self.x_drawing_offset = offset;
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn update_rendering_scales(&mut self) {
        self.ndc_block_scale = self.render_scale / ((self.zoom * 2) + 1) as f32;
        self.ndc_tile_scale = self.ndc_block_scale / 2.0;
        self.ndc_tile_half_scale = self.ndc_tile_scale / 2.0;
    }

    pub fn render_view(&mut self, screen_data: &ScreenData, texture_manager: &mut TextureManager, world: &Arc<RwLock<World>>) {
        self.update_rendering_scales();
        self.handle_camera_panning(screen_data);

        let world = world.read().unwrap();

        let ndc_x_draw_center_offset = self.ndc_block_scale + self.x_drawing_offset;

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
                    let mut draw_cords = iso_cord_tool::casted_to_ndc_cords(self.ndc_block_scale, [x - z, y - z]);
                    draw_cords[0] -= ndc_x_draw_center_offset;

                    draw_cords[0] += self.camera_ndc_offset[0];
                    draw_cords[1] += self.camera_ndc_offset[1];


                    texture_manager.render_block(block_type_at_cord, draw_cords, self.ndc_block_scale);

                    // Render Selector if at center
                    if x == 0 && y == 0 && z == 0 {
                        if block_type_at_cord != BlockType::Air {
                            texture_manager.render_block(BlockType::translucent_red, draw_cords, self.ndc_block_scale);
                        }
                        texture_manager.render_block(BlockType::selector, draw_cords, self.ndc_block_scale);
                    }
                }
            }
        }
    }

    //=====================================
    // Controls
    //=====================================

    fn handle_camera_panning(&mut self, screen_data: &ScreenData) {
        // Update camera offset based off scrolling change
        if screen_data.is_middle_mouse_held() {
            let scrolling_offset = screen_data.get_change_in_mouse_ndc();
            self.camera_ndc_offset[0] += scrolling_offset[0];
            self.camera_ndc_offset[1] += scrolling_offset[1];
        }

        // Correct for camera_ndc_offset

        // Get iso offset amounts
        let iso_offset = iso_cord_tool::ndi_screen_cords_to_iso_cords(self.ndc_block_scale, self.camera_ndc_offset);

        // Iso X camera movment
        if iso_offset[0] > 1.0 {
            self.mod_cords([-1, 0, 0]);
            self.camera_ndc_offset[0] -= self.ndc_tile_scale;
            self.camera_ndc_offset[1] -= self.ndc_tile_half_scale;
        }
        if iso_offset[0] < -1.0 {
            self.mod_cords([1, 0, 0]);
            self.camera_ndc_offset[0] += self.ndc_tile_scale;
            self.camera_ndc_offset[1] += self.ndc_tile_half_scale;
        }

        // Iso Y Cam movment
        if iso_offset[1] > 1.0 {
            self.mod_cords([0, -1, 0]);
            self.camera_ndc_offset[0] += self.ndc_tile_scale;
            self.camera_ndc_offset[1] -= self.ndc_tile_half_scale;
        }

        if iso_offset[1] < -1.0 {
            self.mod_cords([0, 1, 0]);
            self.camera_ndc_offset[0] -= self.ndc_tile_scale;
            self.camera_ndc_offset[1] += self.ndc_tile_half_scale;
        }
    }

    pub fn mouse_wheel_event(&mut self, _x: f32, _y: f32) {
        if _y > 0.0 {
            self.mod_zoom(-1);
        }
        else if _y < 0.0 {
            self.mod_zoom(1)
        }
    }

    pub fn key_down_event(&mut self, keycode: KeyCode) {
        match keycode {
            // Rotate camera
            KeyCode::Q => {
                self.mod_cords([0, 0, -1]);
            }
            KeyCode::E => {
                self.mod_cords([0, 0, 1]);
            }
            
            // Vertical Camera Movement
            KeyCode::S => {
                self.mod_cords([1, 0, 0]);
            }
            KeyCode::W => {
                self.mod_cords([-1, 0, 0])
            }

            // Horozontal Camera Movemnent
            KeyCode::A => {
                self.mod_cords([0, 1, 0]);
            }
            KeyCode::D => {
                self.mod_cords([0, -1, 0])
            }

            KeyCode::R => {
                self.rotate_right();
            }
            _ => {
                
            }
        }
    }

    pub fn mouse_motion_event(&mut self, screen_data: &ScreenData) {
        let current_mouse_ndc = screen_data.get_mouse_ndc();

    }

    pub fn mouse_button_down_event(&mut self, event_manager: &mut GameEventManager, screen_data: &ScreenData, button: MouseButton) {
        if button == MouseButton::Left {
            event_manager.add_world_event(WorldEvent::ModBlock(self.camera_cords, BlockType::Air));
        }
        else if button == MouseButton::Right {
            event_manager.add_world_event(WorldEvent::ModBlock(self.camera_cords, BlockType::Stone));
        }
    }

    pub fn mouse_button_up_event(&mut self, event_manager: &mut GameEventManager, screen_data: &ScreenData, button: MouseButton) {
        
    }

    //=====================================
    // Debug
    //=====================================
    pub fn collect_debug_data(&self, debug_data: &mut DebugData) {
        debug_data.set_camera_cords(self.camera_cords);
        debug_data.set_direction(self.view_direction.to_string());
    }

}