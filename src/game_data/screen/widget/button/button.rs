use rand::rand_core::block;

use crate::game_data::{TextureManager, game_event_manager::{self, event_manager::EventManager, game_event_manager::game_event_manager::GameEvent, prelude::Event}, screen::{ScreenData, render_centered_string_at_ndc, screen_data, screen_mananager, widget::{self, widget::Widget, widget_calculations}}, types::{BlockTexture, FontType, UITextures}};

pub struct Button {
    // Parent Rendering
    parent_pos: [f32; 4],
    parent_scale: [f32; 2],
    
    // Rendering
    needs_resizing: bool,
    external_buffers: [f32; 4],

    pos: [f32; 4],
    scale: [f32; 2],

    prefered_scale: [f32; 2],

    
    // Input
    events: Vec<Event>, // The event that will occer when the button is pressed

    // Apearence
    button_type: UITextures,
    apearence_pos: [f32; 4],
    text_ndc: [f32; 2],
    
    text: Option<String>,
    block_texture: Option<BlockTexture>,
    icon_type: Option<UITextures>,

}

impl Button {
    pub fn new(buffers: [f32; 4]) -> Button {

        let mut button = Button {
            // Parent Rendering
            parent_pos: [0.0; 4],
            parent_scale: [0.0; 2],

            // Rendering
            needs_resizing: true,
            external_buffers: buffers,

            pos: [0.0; 4],
            scale: [0.0; 2],
            
            prefered_scale: [widget_calculations::get_button_scale(); 2],

            // Input 
            events: Vec::new(), 

            // Apearence
            button_type: UITextures::ButtonCircle, 
            apearence_pos: [0.0; 4],
            text_ndc: [0.0; 2],

            text: None,
            block_texture: None, 
            icon_type: None,
        };

        button.size();

        return button;
    }

    pub fn size(&mut self) {
        // Parent
        self.parent_scale = widget_calculations::pos_to_scale(self.parent_pos);
        
        // Self
        self.pos = widget_calculations::buffer_pos(self.parent_pos, self.external_buffers);
        self.scale = widget_calculations::pos_to_scale(self.pos);
        
        // println!("Button Sizing :");
        // println!("  - Sizing : external buffers {:?}", self.external_buffers);
        // println!("  - Sized : pos({:?}), scale({:?})", self.pos, self.scale);

        // Apearence

        self.apearence_pos = widget_calculations::buffer_pos(self.pos, [self.scale[0] / 5.0; 4]);
        self.text_ndc = [
            self.pos[0] + (self.scale[0] / 2.0),
            self.pos[1] - (widget_calculations::get_button_text_scale() / 2.0),
        ];

        self.needs_resizing = false;
    }

    pub fn set_text_scale(&mut self, size: widget_calculations::TextSize) {
        self.prefered_scale[1] = size.get_scale();
    }

    //=====================================
    // Events
    //=====================================

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    } 

    //=====================================
    // Apearence
    //=====================================
    pub fn set_block(&mut self, block_texture: BlockTexture) {
        self.block_texture = Some(block_texture);
    }

    pub fn set_icon(&mut self, icon: UITextures) {
        self.icon_type = Some(icon);
    }

    pub fn set_text(&mut self, text: String) {
        self.text = Some(text);
    }

    fn render_apearence(&self, texture_manager: &mut TextureManager, screen_data: &ScreenData) {
        // Render block
        if let Some(block_texture) = self.block_texture {
            texture_manager.render_block_with_pos(block_texture, self.apearence_pos);
        }
        
        // Render icon
        if let Some(icon) = self.icon_type {
            texture_manager.render_ui_element_with_pos(icon, self.apearence_pos);
        }

        if let Some(text) = &self.text {
            if screen_data.mouse_on_ndc_pos(self.pos) {
                render_centered_string_at_ndc(
                    texture_manager, 
                    text.clone(), 
                    FontType::Basic, 
                    widget_calculations::get_button_text_scale(), 
                    self.text_ndc
                );            
            }
        }
    }

    
}

impl Widget for Button {
    fn get_pos(&self) -> [f32; 4] {
        return self.pos;
    }

    fn get_scale(&self) -> [f32; 2] {
        return self.scale;
    }

    fn get_preffered_scale(&self) -> [f32; 2] {
        return self.prefered_scale;
    }

    fn set_buffers(&mut self, buffers: [f32; 4]) {
        self.external_buffers = buffers;
        self.needs_resizing = true;
    }

    fn set_parent_pos(&mut self, pos: [f32; 4]) {
        self.parent_pos = pos;
        self.needs_resizing = true;
    }
    
    fn size(&mut self) {
        self.size();
    }

    fn render(
        &mut self, 
        texture_manager: &mut TextureManager, 
        screen_data: &ScreenData, 
        game_event_manager: &mut EventManager
    ) {

        if self.needs_resizing {
            self.size();
        }

        let mut button_texture = self.button_type;
        // If mouse is on button
        if screen_data.mouse_on_ndc_pos(self.pos) {
            button_texture = self.button_type.get_pressed_variant(); // Update texture

            // If button was clicked
            if screen_data.was_left_released(){
                for event in &self.events {
                    game_event_manager.add_event(event.clone());
                }    
            }
        }

        // Render button 
        texture_manager.render_ui_element_with_pos(button_texture, self.pos);

        // Render aperence values
        self.render_apearence(texture_manager, screen_data);
    }

}