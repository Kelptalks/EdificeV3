use miniquad::MouseButton;

use crate::game_data::{TextureManager, screen::ScreenData, types::UITextures};

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

}