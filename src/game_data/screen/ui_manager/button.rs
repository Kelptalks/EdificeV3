use miniquad::MouseButton;

use crate::game_data::{TextureManager, screen::{ScreenData, render_centered_string_at_ndi_cords}, types::{BlockType, UITextures}};

pub struct Button { 
    cords: [f32; 2],
    scale: f32,

    is_pressed: bool,
    button_type: UITextures,
}

impl Button {
    pub fn new(cords: [f32; 2], scale: f32, button_type: UITextures) -> Button {
        Button {
            
            cords: cords,
            scale,
            is_pressed: false,

            button_type: button_type,
        }
    }

    pub fn render_button(&self, texture_manager: &mut TextureManager) {
        let button_texture = if self.is_pressed {
            self.button_type
        } else {
            self.button_type.get_pressed_variant()
        };

        texture_manager.render_ui_element(
            button_texture,
            self.cords,
            self.scale,
        );
    }

    pub fn render_block_on_button(&self, texture_manager: &mut TextureManager, block: BlockType) {
        // Render drone on top of button
        let scale = self.scale / 3.0;
        let offset = ((scale * 2.0) - self.scale) / 2.0;
        let draw_location: [f32; 2] = [
            self.cords[0] - offset,
            self.cords[1] - offset
        ];
        texture_manager.render_block(block, draw_location, scale);
    }
    
    pub fn handle_mouse_motion_input(&mut self, screen_data: &ScreenData) { 
        // if mouse is over button
        let mouse_cords = screen_data.get_mouse_ndc_cords();
        if mouse_cords[0] >= self.cords[0] &&
           mouse_cords[0] <= self.cords[0] + self.scale &&
           mouse_cords[1] >= self.cords[1] &&
           mouse_cords[1] <= self.cords[1] + self.scale {
               self.is_pressed = true;
        } else {
            self.is_pressed = false;
        }

    }

    pub fn is_mouse_on_button(&self) -> bool {
        self.is_pressed
    }

    pub fn get_ndc_cords(&self) -> [f32; 2] {
        return self.cords;
    }

    pub fn get_scale(&self) -> f32 {
        return self.scale;
    }

}