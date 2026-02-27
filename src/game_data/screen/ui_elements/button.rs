use miniquad::MouseButton;

use crate::game_data::{TextureManager, screen::{ScreenData, render_centered_string_at_ndc, text::render_string_at_ndc, ui_elements::panel::Panel}, types::{BlockTexture, FontType, UITextures}};

pub struct Button { 
    // Rendering
    ndc: [f32; 2],
    scale: f32,

    // Controls
    is_mouse_on: bool,

    // Apearence
    button_type: UITextures,
    block_type: Option<BlockTexture>,

    // Text
    text: Option<String>,
    text_panel: Panel,
    text_panel_padding: f32,
    text_scale: f32,
    text_font: FontType,

}

impl Button {
    
    //=====================================
    // Constructors
    //=====================================
    // Create button at a location
    pub fn new(cords: [f32; 2], scale: f32, button_type: UITextures) -> Button {
        Button {
            // Rendering
            ndc: cords,
            scale,

            // Controls
            is_mouse_on: false,

            // Apearence
            button_type: button_type,
            block_type: None,
            
            // Text
            text: None,
            text_panel: Panel::new_blank(),
            text_panel_padding: 0.015,
            text_scale: 0.025,
            text_font: FontType::Basic,
            
        }
    }

    // Create a button that has no cords or scale
    pub fn new_blank(button_type: UITextures) -> Button {
        Button {
            // Rendering
            ndc: [0.0, 0.0], 
            scale: 0.0, 

            // Controls
            is_mouse_on: false, 

            // Apearence
            button_type: button_type,
            block_type: None,

            // Text
            text: None,
            text_panel: Panel::new_blank(),
            text_panel_padding: 0.015,
            text_scale: 0.025,
            text_font: FontType::Basic,
        }
    }

    //=====================================
    // Getters and setters
    //=====================================

    pub fn is_mouse_on_button(&self) -> bool {
        self.is_mouse_on
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
    pub fn set_block(&mut self, block: BlockTexture) {
        self.block_type = Some(block);
    }
    pub fn set_text(&mut self, text: String) {
        self.text_scale = 0.025;


        self.text_panel.set_ndc_scale([
            (self.text_scale * (text.len() + 1) as f32) + (self.text_panel_padding * 2.0), 
            (self.text_panel_padding * 2.0) + self.text_scale,
        ]);
        self.text_panel.set_tile_ndc_scale(self.text_scale / 6.0);

        self.text = Some(text);
    }

    //=====================================
    // Rendering
    //=====================================

    pub fn render_button(&mut self, texture_manager: &mut TextureManager, screen_data: &ScreenData) {
        // if mouse is over button
        let mouse_cords = screen_data.get_mouse_ndc();
        if mouse_cords[0] >= self.ndc[0] &&
           mouse_cords[0] <= self.ndc[0] + self.scale &&
           mouse_cords[1] >= self.ndc[1] &&
           mouse_cords[1] <= self.ndc[1] + self.scale {
            self.is_mouse_on = true;
        } else {
            self.is_mouse_on = false;
        }
        
        // Render the button itself
        let button_texture = if self.is_mouse_on {
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
            if self.is_mouse_on {
                let mut ndc = screen_data.get_mouse_ndc();
                ndc[0] += self.text_panel_padding * 2.0;
                ndc[1] -= self.text_scale / 2.0;

                let panel_ndc = [
                    ndc[0] - self.text_panel_padding,
                    ndc[1] - self.text_panel_padding,
                ];
                self.text_panel.set_ndc(panel_ndc);
                self.text_panel.render(texture_manager);

                render_string_at_ndc(
                    texture_manager, 
                    string.to_string(), 
                    self.text_font, 
                    self.text_scale, 
                    ndc
                );
            }
        }
    }

}