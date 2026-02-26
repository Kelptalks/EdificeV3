use std::sync::{Arc, RwLock};

use image::flat::View;
use miniquad::{KeyCode, MouseButton};

use crate::game_data::{TextureManager, World, debuging::debug_data::DebugData, game_event_manager::{game_event_manager::{self, GameEventManager}, world_event_manager::world_event_manager::WorldEvent}, screen::{ScreenData, camera_data::Direction, iso_cord_tool, play_view::{gui::building_gui_manager::{self, BuildingGUIManager}, play_view_data::{self, PlayViewData}, world_rendering::play_block::{self, PlayBlock}}, screen_data, text}, types::BlockTexture};

static MAX_VIEW_DISTANCE: i32 = 20;
static MIN_VIEW_DISTANCE: i32 = 0;

pub struct PlayWorldRender {
    ndc_cords: [f32; 2],


    // World
    zoom: i32,
    camera_ndc_offset: [f32; 2],
    render_scale: f32,

    // Cached scale values
    ndc_block_scale: f32,
    ndc_tile_scale: f32,
    ndc_tile_half_scale: f32,
}

impl PlayWorldRender {
    pub fn new() -> PlayWorldRender {
        PlayWorldRender {
            // Ndc Cords
            ndc_cords: [0.0, 0.0],

            // Rendering
            zoom: 10,
            camera_ndc_offset: [0.0, 0.0],
            render_scale: 0.5,

            ndc_block_scale: 0.0,
            ndc_tile_scale: 0.0,
            ndc_tile_half_scale: 0.0,
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================

    // Mod amount of blocks in view
    pub fn mod_zoom(&mut self, zoom_mod: i32) {
        let new_zoom = self.zoom + zoom_mod;
        if new_zoom > MIN_VIEW_DISTANCE && new_zoom < MAX_VIEW_DISTANCE {
            self.zoom = new_zoom;
        }
    }

    pub fn set_ndc_center_cords(&mut self, offset: [f32; 2]) {
        self.ndc_cords = offset;
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn update_rendering_scales(&mut self) {
        self.ndc_block_scale = self.render_scale / ((self.zoom * 2) + 1) as f32;
        self.ndc_tile_scale = self.ndc_block_scale / 2.0;
        self.ndc_tile_half_scale = self.ndc_tile_scale / 2.0;
    }

    pub fn render_view(&mut self, 
        screen_data: &ScreenData, 
        play_view_data: &mut PlayViewData,
        texture_manager: &mut TextureManager,
        world: &Arc<RwLock<World>>
    ) {
        self.update_rendering_scales();
        self.handle_camera_panning(play_view_data, screen_data);

        let world = world.read().unwrap();

        let ndc_x_draw_center_offset = self.ndc_block_scale - self.ndc_cords[0];

        let direction_offsets = play_view_data.get_view_direction_offsets();

        let camera_cords = play_view_data.get_world_cords();

        // Loop through blocks in zoom
        for z in -self.zoom..=self.zoom {
            for y in -self.zoom..=self.zoom {
                for x in -self.zoom..=self.zoom {
                    let world_block_cords = [
                        camera_cords[0] + (x * direction_offsets[0]),
                        camera_cords[1] + (y * direction_offsets[1]),
                        camera_cords[2] + z,
                    ];

                    // Block Type
                    let block_type_at_cord = BlockTexture::from_id(world.get_world_value(world_block_cords));
                    
                    // Draw Cords
                    let mut draw_cords = iso_cord_tool::casted_to_ndc_cords(self.ndc_block_scale, [x - z, y - z]);
                    draw_cords[0] -= ndc_x_draw_center_offset;

                    draw_cords[0] += self.camera_ndc_offset[0];
                    draw_cords[1] += self.camera_ndc_offset[1];


                    // Create play block
                    let mut play_block = PlayBlock::new_blank();

                    // World
                    play_block.block_type = block_type_at_cord;
                    play_block.block_world_cords = world_block_cords;

                    // Rendering
                    play_block.rendering_block_cords = [x, y, z];
                    play_block.draw_cords = draw_cords;
                    play_block.ndc_block_scale = self.ndc_block_scale;


                    play_block.render_block(texture_manager);
                    play_block.render_cursor(texture_manager);
                    
                    for area in play_view_data.get_areas_selected() {
                        play_block.render_area_selection(texture_manager, area);
                    }


                }
            }
        }
    }

    //=====================================
    // Controls
    //=====================================

    fn handle_camera_panning(&mut self, play_view_data: &mut PlayViewData, screen_data: &ScreenData) {
        // Update camera offset based off scrolling change
        if screen_data.is_middle_mouse_held() {
            let scrolling_offset = screen_data.get_change_in_mouse_ndc();
            self.camera_ndc_offset[0] += scrolling_offset[0];
            self.camera_ndc_offset[1] += scrolling_offset[1];
        }

        // Correct for camera_ndc_offset

        // Get iso offset amounts
        let iso_offset = iso_cord_tool::ndi_screen_cords_to_iso_cords(self.ndc_tile_scale, self.camera_ndc_offset);

        // Iso X camera movment
        if iso_offset[0] > 1.0 {
            play_view_data.mod_world_cords([-1, 0, 0]);
            self.camera_ndc_offset[0] -= self.ndc_tile_scale;
            self.camera_ndc_offset[1] -= self.ndc_tile_half_scale;
        }
        if iso_offset[0] < -1.0 {
            play_view_data.mod_world_cords([1, 0, 0]);
            self.camera_ndc_offset[0] += self.ndc_tile_scale;
            self.camera_ndc_offset[1] += self.ndc_tile_half_scale;
        }

        // Iso Y Cam movment
        if iso_offset[1] > 1.0 {
            play_view_data.mod_world_cords([0, -1, 0]);
            self.camera_ndc_offset[0] += self.ndc_tile_scale;
            self.camera_ndc_offset[1] -= self.ndc_tile_half_scale;
        }

        if iso_offset[1] < -1.0 {
            play_view_data.mod_world_cords([0, 1, 0]);
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

    pub fn key_down_event(&mut self, play_view_data: &mut PlayViewData, keycode: KeyCode) {
        match keycode {
            // Rotate camera
            KeyCode::Q => {
                play_view_data.rotate_left();
            }
            KeyCode::E => {
                play_view_data.rotate_right();
            }
            
            // X Axis
            KeyCode::S => {
                play_view_data.mod_world_cords([1, 0, 0]);
            }
            KeyCode::W => {
                play_view_data.mod_world_cords([-1, 0, 0])
            }

            // Y Axis
            KeyCode::A => {
                play_view_data.mod_world_cords([0, 1, 0]);
            }
            KeyCode::D => {
                play_view_data.mod_world_cords([0, -1, 0])
            }

            // Z Axis
            KeyCode::LeftShift => {
                play_view_data.mod_world_cords([0, 0, -1]);
            }
            KeyCode::Space => {
                play_view_data.mod_world_cords([0, 0, 1]);
            }
            _ => {
                
            }
        }
    }

    pub fn mouse_motion_event(&mut self, screen_data: &ScreenData) {
        let current_mouse_ndc = screen_data.get_mouse_ndc();

    }

    pub fn mouse_button_down_event(&mut self, event_manager: &mut GameEventManager, play_view_data: &PlayViewData, screen_data: &ScreenData, button: MouseButton) {
        
    }

    pub fn mouse_button_up_event(&mut self, event_manager: &mut GameEventManager, screen_data: &ScreenData, button: MouseButton) {
        
    }

    //=====================================
    // Debug
    //=====================================

}