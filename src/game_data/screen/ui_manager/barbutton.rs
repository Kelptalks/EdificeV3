use miniquad::MouseButton;

use crate::game_data::{TextureManager, screen::{ScreenData, render_centered_string_at_ndi_cords, ui_manager::button}, types::UITextures};

pub struct BarButton {
    cords: [f32; 2],
    scale: f32,
    length: u32,

    is_pressed: bool,
    text : String,
}

impl BarButton {
    pub fn new(cords: [f32; 2], scale: f32, length: u32) -> BarButton {
        BarButton {
            cords: cords,
            scale,
            length,
            is_pressed: false,

            text: String::new(),
        }
    }

    pub fn new_with_text(cords: [f32; 2], scale: f32, length: u32, text: String) -> BarButton { 
        BarButton {
            cords: cords,
            scale,
            length,
            is_pressed: false,
            text: text,
        }
    }

    pub fn render_button(&self, texture_manager: &mut TextureManager) {

        // Render the button itself
        for i in 0..self.length {
            let draw_x = self.cords[0] + (i as f32 * self.scale);
            let draw_location = [draw_x, self.cords[1]];
            let mut button_texture = UITextures::BarButtonCenter;
            // Button start
            if i == 0 {
                button_texture = UITextures::BarButtonLeft;
            }
            // button end
            else if i == self.length - 1{
                button_texture = UITextures::BarButtonRight;
            }

            // if button press
            if self.is_pressed {
                button_texture = button_texture.get_pressed_variant();
            }

            texture_manager.render_ui_element(button_texture, draw_location, self.scale);
        }


        // calculate string center cords
        let x_button_center_cor = self.cords[0] + (self.get_x_scale() / 2.0);
        let y_button_center_cor = self.cords[1] + (self.scale) / 3.0;
        let button_text_scale = self.scale * 0.3;
        render_centered_string_at_ndi_cords(texture_manager,
            self.text.to_string(),
            "Basic".to_string(),
            button_text_scale,
            [x_button_center_cor, y_button_center_cor],
        );

    }

    pub fn handle_mouse_button_down(&mut self, screen_data: &mut ScreenData, button: MouseButton) { 

    }

    pub fn handle_mouse_motion_input(&mut self, screen_data: &ScreenData) { 
        // if mouse is over button
        let mouse_cords = screen_data.get_mouse_ndc_cords();
        let button_x_scale = self.scale * self.length as f32;
        
        if mouse_cords[0] >= self.cords[0] &&
           mouse_cords[0] <= self.cords[0] + button_x_scale &&
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

    pub fn get_pos(&self) -> [f32; 2] {
        self.cords
    }

    pub fn get_x_scale(&self) -> f32 {
        return self.scale * self.length as f32;
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}