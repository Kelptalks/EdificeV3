use std::{fmt::format, i32};

use miniquad::MouseButton;

use crate::game_data::{TextureManager, screen::{Button, ScreenData, render_centered_string_at_ndc}, texture_manager, types::UITextures};

pub struct Stepper {
    // Value controls
    value: i32,
    value_max: i32,
    value_min: i32,
    increment: i32,

    // Rendering
    scale: f32,
    text_scale: f32,
    ndc: [f32; 2],
    text: String,

    // Buttons
    decrease_button: Button,
    increase_button: Button,
}

impl Stepper {
    pub fn new_blank() -> Stepper {
        return Stepper {
            value: 0, 
            value_max: i32::MAX, 
            value_min: i32::MIN, 
            increment: 1,

            scale: 0.0, 
            text_scale: 0.0,
            ndc: [0.0, 0.0],
            text: "".to_string(), 
            decrease_button: Button::new_blank_with_texture(UITextures::ButtonLeftArrow),
            increase_button: Button::new_blank_with_texture(UITextures::ButtonRightArrow),
        }
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn render(&mut self, texture_manager: &mut TextureManager, screen_data: &ScreenData) {
        // Render text
        render_centered_string_at_ndc(texture_manager, 
            self.text.clone(), 
            crate::game_data::types::FontType::Basic, 
            self.text_scale, 
            [self.ndc[0], self.ndc[1] + self.scale / 3.0]
        );

        let number = format!("{}", self.value).to_string();
        render_centered_string_at_ndc(texture_manager, 
            number, 
            crate::game_data::types::FontType::Basic, 
            self.text_scale, 
            [self.ndc[0], self.ndc[1] + self.scale]
        );

        // Render buttons
        self.decrease_button.render_button(texture_manager, screen_data);
        self.increase_button.render_button(texture_manager, screen_data);
    }

    pub fn re_calculate_rendering_values(&mut self) {
        self.text_scale = self.scale * 0.3;
        let button_spacing = (self.text.len() as f32 * self.text_scale) + self.scale;
        
        let left_button_ndc = [
            self.ndc[0] - (button_spacing/2.0) - self.scale,
            self.ndc[1]
        ];
        self.decrease_button.set_ndc(left_button_ndc);

        let right_button_ndc = [
            self.ndc[0] + button_spacing/2.0,
            self.ndc[1]
        ];
        self.increase_button.set_ndc(right_button_ndc);


        self.decrease_button.set_scale(self.scale);
        self.increase_button.set_scale(self.scale);
    }

    //=====================================
    // Setters / Getters
    //=====================================

    // Rendering
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
        self.re_calculate_rendering_values();
    }
    pub fn set_ndc(&mut self, ndc: [f32; 2]) {
        self.ndc = ndc;
        self.re_calculate_rendering_values();
    }
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.re_calculate_rendering_values();
    }

    // Value
    pub fn set_max_value(&mut self, max_value: i32) {
        self.value_max = max_value;
    }
    pub fn set_min_value(&mut self, min_value: i32) {
        self.value_min = min_value;
    }
    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }
    pub fn set_increment(&mut self, increment: i32) {
        self.increment = increment;
    }

    pub fn get_value(&self) -> i32 {
        return self.value;
    }

    //=====================================
    // Controls
    //=====================================
    
    pub fn handle_mouse_button_down(&mut self, mouse_button: MouseButton) {
        if mouse_button == MouseButton::Left {
            if self.decrease_button.is_mouse_on_button() {
                if self.value > self.value_min {
                    self.value -= self.increment;
                }
            }
            else if self.increase_button.is_mouse_on_button() {
                if self.value < self.value_max {
                    self.value += self.increment;
                }
            }
        }
    }
}