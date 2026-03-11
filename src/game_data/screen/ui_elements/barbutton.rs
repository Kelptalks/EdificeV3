use miniquad::MouseButton;

use crate::game_data::{TextureManager, screen::{ScreenData, render_centered_string_at_ndc, ui_elements::button, widget::widget_calculations::TextSize}, types::{FontType, UITextures}};

pub struct BarButton {
    ndc: [f32; 2],
    scale: f32,
    length: u32,

    is_pressed: bool,
    text: String,
    text_scale: f32,
}

impl BarButton {
    //=====================================
    // Constructors
    //=====================================
    pub fn new(cords: [f32; 2], scale: f32, length: u32) -> BarButton {
        BarButton {
            ndc: cords,
            scale,
            length,
            is_pressed: false,
            text: String::new(),
            text_scale: TextSize::Small.get_scale(),
        }
    }

    pub fn new_blank() -> BarButton{
        BarButton {
            ndc: [0.0, 0.0],
            scale: 0.0,
            length: 0,
            is_pressed: false,
            text: String::new(),
            text_scale: TextSize::Small.get_scale(),
        }
    }

    pub fn new_with_text(cords: [f32; 2], scale: f32, length: u32, text: String) -> BarButton {
        BarButton {
            ndc: cords,
            scale,
            length,
            is_pressed: false,
            text: text,
            text_scale: TextSize::Small.get_scale(),
        }
    }

    //=====================================
    // Getters / Setters
    //=====================================

    pub fn is_mouse_on_button(&self) -> bool {
        self.is_pressed
    }

    pub fn get_pos(&self) -> [f32; 2] {
        self.ndc
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn set_ndc(&mut self, ndc: [f32; 2]) {
        self.ndc = ndc;
    }

    pub fn set_length(&mut self, length: u32) {
        self.length = length
    }

    pub fn get_x_scale(&self) -> f32 {
        return self.scale * self.length as f32;
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn set_text_scale(&mut self, size: TextSize) {
        self.text_scale = size.get_scale();
    }

//=====================================
    // Rendering
    //=====================================
    pub fn render_button(&self, texture_manager: &mut TextureManager) {

        // Render the button itself
        for i in 0..self.length {
            let draw_x = self.ndc[0] + (i as f32 * self.scale);
            let draw_location = [draw_x, self.ndc[1]];
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
        let x_button_center_cor = self.ndc[0] + (self.get_x_scale() / 2.0);
        let y_button_center_cor = self.ndc[1] + (self.scale) / 2.0;
        let button_text_scale = self.text_scale;
        render_centered_string_at_ndc(texture_manager,
            self.text.to_string(),
            FontType::Basic,
            button_text_scale,
            [x_button_center_cor, y_button_center_cor],
        );

    }


    //=====================================
    // Controls
    //=====================================

    pub fn handle_mouse_button_down(&mut self, screen_data: &mut ScreenData, button: MouseButton) { 

    }

    pub fn handle_mouse_motion_input(&mut self, screen_data: &ScreenData) { 
        // if mouse is over button
        let mouse_cords = screen_data.get_mouse_ndc();
        let button_x_scale = self.scale * self.length as f32;
        
        if mouse_cords[0] >= self.ndc[0] &&
           mouse_cords[0] <= self.ndc[0] + button_x_scale &&
           mouse_cords[1] >= self.ndc[1] &&
           mouse_cords[1] <= self.ndc[1] + self.scale {
               self.is_pressed = true;
        } else {
            self.is_pressed = false;
        }

    }
}