use miniquad::MouseButton;

use crate::game_data::{TextureManager, screen::{ScreenData, render_centered_string_at_ndc}, types::{BlockType, FontType, UITextures}};

pub struct Button { 
    // Rendering
    ndc: [f32; 2],
    scale: f32,

    // Controls
    is_pressed: bool,

    // Apearence
    button_type: UITextures,
    block_type: Option<BlockType>,
    text: Option<String>,

}

impl Button {
    
    //=====================================
    // Init
    //=====================================

    
    // Create button at a location
    pub fn new(cords: [f32; 2], scale: f32, button_type: UITextures) -> Button {
        Button {
            // Rendering
            ndc: cords,
            scale,

            // Controls
            is_pressed: false,

            // Apearence
            button_type: button_type,
            block_type: None,
            text: None,
            
        }
    }

    // Create a button that has no cords or scale
    pub fn new_blank(button_type: UITextures) -> Button {
        Button {
            // Rendering
            ndc: [0.0, 0.0], 
            scale: 0.0, 

            // Controls
            is_pressed: false, 

            // Apearence
            button_type: button_type,
            block_type: None,
            text: None,
        }
    }

    //=====================================
    // Getters and setters
    //=====================================

    pub fn is_mouse_on_button(&self) -> bool {
        self.is_pressed
    }

    // Cords
    pub fn set_ndc(&mut self, ndc: [f32; 2]) {
        self.ndc = ndc;
    }
    pub fn get_ndc(&self) -> [f32; 2] {
        return self.ndc;
    }

    // Scale
    pub fn get_scale(&self) -> f32 {
        return self.scale;
    }
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    // Apearence
    pub fn set_block(&mut self, block: BlockType) {
        self.block_type = Some(block);
    }
    pub fn set_text(&mut self, text: String) {
        self.text = Some(text);
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn render_button(&self, texture_manager: &mut TextureManager) {
        // Render the button itself
        let button_texture = if self.is_pressed {
            self.button_type
        } else {
            self.button_type.get_pressed_variant()
        };

        texture_manager.render_ui_element(
            button_texture,
            self.ndc,
            self.scale,
        );


        // Render block on top if assigned one
        if let Some(block) = self.block_type {
            // Render block on top of button
            let scale = self.scale / 3.0;
            let offset = ((scale * 2.0) - self.scale) / 2.0;
            let draw_location: [f32; 2] = [
                self.ndc[0] - offset,
                self.ndc[1] - offset
            ];
            texture_manager.render_block(block, draw_location, scale);
        }

        // If has text and mouse is on button render text above button
        if let Some(string) = &self.text {
            if self.is_pressed {
                let ndc = [
                    self.ndc[0],
                    self.ndc[1] - self.scale
                ];

                render_centered_string_at_ndc(texture_manager, 
                    string.to_string(), 
                    FontType::Basic, 
                    self.scale / 4.0, 
                    ndc
                );
            }
        }
    }
    
    //=====================================
    // Controls
    //=====================================

    pub fn handle_mouse_motion_input(&mut self, screen_data: &ScreenData) { 
        // if mouse is over button
        let mouse_cords = screen_data.get_mouse_ndc();
        if mouse_cords[0] >= self.ndc[0] &&
           mouse_cords[0] <= self.ndc[0] + self.scale &&
           mouse_cords[1] >= self.ndc[1] &&
           mouse_cords[1] <= self.ndc[1] + self.scale {
               self.is_pressed = true;
        } else {
            self.is_pressed = false;
        }

    }

}