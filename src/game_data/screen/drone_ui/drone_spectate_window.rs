use std::sync::{Arc, RwLock};

use miniquad::MouseButton;

use crate::game_data::{TextureManager, World, screen::{ScreenData, iso_cord_tool}, types::{BlockType, DroneUITexture}};

pub struct SpectateWindow {
    ndc_cords: [f32; 2],
    end_ndc_cords: [f32; 2],
    scale: f32,
    y_scale: f32,
    visible: bool,

    // Drone Data
    drone_world_cords: [i32; 3],
    block_display_window_ndc_cords: [f32; 2],
    spectate_zoom: i32,
}

impl SpectateWindow {
    pub fn new() -> SpectateWindow {
        SpectateWindow {
            // rendering data
            ndc_cords: [0.0, 0.0],
            end_ndc_cords: [0.0, 0.0],
            scale: 0.5,
            y_scale: 0.0,
            visible: false,


            // Drone spectate data
            drone_world_cords: [0, 0, 0],
            block_display_window_ndc_cords: [0.37, 0.5],
            spectate_zoom: 2,
        }
    }


    //=====================================
    // Setters / Getters
    //=====================================

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    pub fn is_visible(&self) -> bool {
        return self.visible;
    }


    pub fn set_drone_world_cords(&mut self, world_cords: [i32; 3]) {
        self.drone_world_cords = world_cords;
    }

    pub fn set_ndc_cords(&mut self, ndc_cords: [f32; 2]) {
        self.ndc_cords = ndc_cords;
        let y_scale = DroneUITexture::DroneSpectateWindow.get_y_to_x_ratio() * self.scale;
        self.y_scale = y_scale;

        self.end_ndc_cords = [self.ndc_cords[0] + self.scale, self.ndc_cords[1] + y_scale];

    }


    
    //=====================================
    // Rendering
    //=====================================

    pub fn render(&self, texture_manager: &mut TextureManager, world: &World) {
        texture_manager.render_drone_ui_element(DroneUITexture::DroneSpectateWindow, self.ndc_cords, self.scale);

        let scale = self.y_scale / (6.5 * self.spectate_zoom as f32);
        let x_draw_location = ((self.block_display_window_ndc_cords[0] * self.scale) + self.ndc_cords[0]) - scale;
        let y_draw_location = ((self.block_display_window_ndc_cords[1] * self.y_scale) + self.ndc_cords[1]) - scale;



        // Render blocks around drone
        for z_block_cor in -self.spectate_zoom..=self.spectate_zoom {
            for x_block_cor in -self.spectate_zoom..=self.spectate_zoom {
                for y_block_cor in -self.spectate_zoom..=self.spectate_zoom {
                    let draw_offset = iso_cord_tool::casted_to_ndc_cords(
                        scale,
                        [x_block_cor - z_block_cor, y_block_cor - z_block_cor]
                    );

                    let final_draw_location = [draw_offset[0] + x_draw_location, draw_offset[1] + y_draw_location];

                    let block_cords = [
                        self.drone_world_cords[0] + x_block_cor,
                        self.drone_world_cords[1] + y_block_cor,
                        self.drone_world_cords[2] + z_block_cor,
                    ];
                    let block = BlockType::from_id(world.get_world_value(block_cords));

                    texture_manager.render_block(block, final_draw_location, scale);
                }
            }   

        }
    }

    //=====================================
    // Controls
    //=====================================

    pub fn handle_motion_event(&mut self, screen_data: &ScreenData) {
        // if on mini_window

    }

    pub fn mouse_on_x(&self, pixel_mouse_cords_on_window: [f32; 2]) -> bool {
        if pixel_mouse_cords_on_window[0] > 254.0 && pixel_mouse_cords_on_window[1] < 8.0 {
            return true;
        }
        else {
            return false;
        }
    }

    pub fn mouse_on_move_bar(&self, pixel_mouse_cords_on_window: [f32; 2]) -> bool {
        if pixel_mouse_cords_on_window[1] < 8.0 {
            if pixel_mouse_cords_on_window[0] > 193.0 && pixel_mouse_cords_on_window[0] < 254.0 {
                return true;
            }
        }

        return false;
    }
    
    pub fn mouse_on_left_arrow(&self, pixel_mouse_cords_on_window: [f32; 2]) -> bool {
        let in_x_range = pixel_mouse_cords_on_window[0] < 23.0;
        let in_y_range = pixel_mouse_cords_on_window[1] > 174.0;
        
        if in_x_range && in_y_range {
            return true;
        }
        else {
            return false;
        }
    }

    pub fn mouse_on_right_arrow(&self, pixel_mouse_cords_on_window: [f32; 2]) -> bool {
        let in_x_range = pixel_mouse_cords_on_window[0] > 170.0 && pixel_mouse_cords_on_window[0] < 192.0;
        let in_y_range = pixel_mouse_cords_on_window[1] > 174.0;
        
        if in_x_range && in_y_range {
            return true;
        }
        else {
            return false;
        }
    }

    pub fn mouse_button_down_event(&mut self, button: MouseButton, screen_data: &ScreenData) {
        // Gotta calculate if mouse is on window based off scale using src rect y to x racio

        if button == MouseButton::Left {
            let mouse_ndc = screen_data.get_mouse_ndc_cords();

            let x_in_window = mouse_ndc[0] > self.ndc_cords[0] && mouse_ndc[0] < self.end_ndc_cords[0];
            let y_in_window = mouse_ndc[1] > self.ndc_cords[1] && mouse_ndc[1] < self.end_ndc_cords[1];

            if x_in_window && y_in_window {

                // Calculate pixel cords for clicking buttons
                let src_rect = DroneUITexture::DroneSpectateWindow.ui_texture_to_sprite_sheet_src_rect();
                let x_mouse_pixel_cords_on_window = ((mouse_ndc[0] - self.ndc_cords[0]) / self.scale) * src_rect[2] as f32;
                let y_mouse_pixel_cords_on_window = ((mouse_ndc[1] - self.ndc_cords[1]) / self.y_scale) * src_rect[3] as f32;
                let pixel_mouse_cords_on_window = [x_mouse_pixel_cords_on_window, y_mouse_pixel_cords_on_window];


                if self.mouse_on_x(pixel_mouse_cords_on_window) {
                    self.visible = false;
                }
                else if self.mouse_on_move_bar(pixel_mouse_cords_on_window) {
                    println!("on bar");
                }
                else if self.mouse_on_right_arrow(pixel_mouse_cords_on_window) {
                    if self.spectate_zoom > 1 {
                        self.spectate_zoom -= 1;
                    }
                }
                else if self.mouse_on_left_arrow(pixel_mouse_cords_on_window) {
                    if self.spectate_zoom <= 3{
                        self.spectate_zoom += 1;
                    }
                }
            }
        }
    }


}