use miniquad::{KeyCode, MouseButton};

use crate::game_data::{screen::renderer::{iso_cord_tool, Camera, CameraData}, GameData, Types::BlockType};

pub struct CameraControls {
    mouse_cords: [f32; 2],
    ndc_mouse_cords: [f32; 2],
    zoom_speed: f32,
}

impl CameraControls {
    pub fn new() -> Self {
        Self {
            mouse_cords: [0.0, 0.0],
            ndc_mouse_cords: [0.0, 0.0],
            zoom_speed: 0.06, 
        }
    }

    pub fn update_mouse_cords(&mut self, camera_data : &CameraData,  mouse_cords: [f32; 2]) {
        self.mouse_cords = mouse_cords;
        
        let screen_rez = camera_data.get_window_rez();

        let ndc_x_cor = (mouse_cords[0] / screen_rez[0]) * 2.0 - 1.0;
        let ndc_y_cor = (mouse_cords[1] / screen_rez[1]) * 2.0 - 1.0;


        self.ndc_mouse_cords = [ndc_x_cor, ndc_y_cor];


        // Debugging
        //println!("Mouse cords: ({}, {})", mouse_cords[0], mouse_cords[1]);
        //println!("NDC Mouse cords: ({}, {})", ndc_x_cor, ndc_y_cor);

    }

    pub fn handle_mouse_inputs(&mut self, camera : &mut Camera, button: MouseButton)
    {
        let camera_data = camera.get_camera_data();
        let window_rez = camera_data.get_window_rez();

        match button {
            MouseButton::Left => {
                let draw_offset = camera_data.get_draw_offset();
                let ndc_mouse_cords = self.ndc_mouse_cords;
                let render_scale = camera_data.get_render_scale();
                let tile_pixel_scale = camera_data.get_tile_render_scale();

                // Apply camera offset to NDC coordinates
                let ndc_world_mouse_cords = [
                    self.ndc_mouse_cords[0] - draw_offset[0],
                    self.ndc_mouse_cords[1] - draw_offset[1]
                ];

                // Calculate the actual scale used for rendering (render_scale * tile_pixel_scale)
                let actual_scale = render_scale * tile_pixel_scale;
                let iso_cords = iso_cord_tool::ndi_screen_cords_to_iso_cords(actual_scale, ndc_world_mouse_cords, window_rez);

                let casted_tile = camera.get_mut_casted_chunk_manager().get_tile_at_iso_cords(iso_cords);

                if let Some(tile) = casted_tile {
                    println!("Found tile at iso cords");
                    tile.get_mut_triangles()[0].add_texture(BlockType::CopperOre, crate::game_data::Types::BlockTriangle::LeftTop);
                } else {
                    println!("No tile found at iso cords");
                }
            }
            MouseButton::Right => {

            }
            _ => {}
        }
    }
    
    pub fn handle_key_inputs(&self, camera_data : &mut CameraData, keycode: KeyCode)
    {
        match keycode {
            KeyCode::W | KeyCode::Up => {
                camera_data.mod_y_cam_cor(0.01);
            }
            KeyCode::A | KeyCode::Right => {
                camera_data.mod_x_cam_cor(0.01);
            }   
            KeyCode::S | KeyCode::Down => {
                camera_data.mod_y_cam_cor(-0.01);
            }
            KeyCode::D | KeyCode::Left => {
                camera_data.mod_x_cam_cor(-0.01);
            }
            _ => {}
        }
        
    }

    pub fn handle_scroll_input(&self, camera_data : &mut CameraData, y_scroll_distance: f32) {
        if (y_scroll_distance > 0.0) {
            camera_data.mod_scale(1.0 + self.zoom_speed);
        }
        else if (y_scroll_distance < 0.0) {
            camera_data.mod_scale(1.0 - self.zoom_speed);
        }
    }
}
