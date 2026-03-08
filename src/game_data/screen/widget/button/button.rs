use rand::rand_core::block;

use crate::game_data::{TextureManager, game_event_manager::{self, game_event_manager::{Event, GameEventManager}}, screen::{ScreenData, widget::{self, widget::Widget}}, types::{BlockTexture, UITextures}};

pub struct Button {
    // Rendering
    pos: [f32; 4],
    buffered_pos: [f32; 4],
    external_buffers: [f32; 4],
    scale: [f32; 2],
    prefered_scale: [f32; 2],

    
    // Input
    event: Event, // The event that will occer when the button is pressed

    // Apearence
    button_type: UITextures,
    apearence_pos: [f32; 4],
    
    block_texture: Option<BlockTexture>,
    icon_type: Option<UITextures>,

}

impl Button {
    pub fn new(event: Event, buffers: [f32; 4]) -> Button {

        let button = Button {
            // Rendering
            pos: [0.0; 4],
            buffered_pos: [0.0; 4],
            external_buffers: buffers,
            scale: [0.0; 2],
            prefered_scale: [0.1; 2],

            // Input 
            event: event, 

            // Apearence
            button_type: UITextures::ButtonCircle, 
            apearence_pos: [0.0; 4],

            block_texture: None, 
            icon_type: None,
        };

        return button;
    }

    pub fn resize(&mut self) {

        self.buffered_pos = [
            self.pos[0] + self.external_buffers[0],
            self.pos[1] + self.external_buffers[1],
            self.pos[2] - self.external_buffers[2],
            self.pos[3] - self.external_buffers[3],
        ];

        let buffered_scale = [
            self.buffered_pos[2] - self.buffered_pos[0],
            self.buffered_pos[3] - self.buffered_pos[1],
        ];
        
        // Calculate aperence pos based of scale
        let apearence_buffer = [
            buffered_scale[0] * 0.8,
            buffered_scale[1] * 0.8,
        ];

        self.apearence_pos = [
            self.buffered_pos[0] + apearence_buffer[0],
            self.buffered_pos[1] + apearence_buffer[1],
            self.buffered_pos[2] - apearence_buffer[0],
            self.buffered_pos[3] - apearence_buffer[1],
        ];
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

    fn render_apearence(&self, texture_manager: &mut TextureManager ) {
        

        // Render block
        if let Some(block_texture) = self.block_texture {
            texture_manager.render_block_with_pos(block_texture, self.apearence_pos);
        }
        
        // Render icon
        if let Some(icon) = self.icon_type {
            texture_manager.render_ui_element_with_pos(icon, self.apearence_pos);
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

    fn has_prefered_scale(&self) -> bool {
        return true;
    }

    fn get_prefered_scale(&self) -> [f32; 2] {
        return self.prefered_scale;
    }

    fn set_pos(&mut self, pos: [f32; 4]) {
        self.pos = pos;
        self.resize();
    }

    

    fn render(
        &self, 
        texture_manager: &mut TextureManager, 
        screen_data: &ScreenData, 
        game_event_manager: &mut GameEventManager
    ) {
        let mut button_texture = self.button_type;
        // If mouse is on button
        if screen_data.mouse_on_ndc_pos(self.buffered_pos) {
            button_texture = self.button_type.get_pressed_variant(); // Update texture

            // If button was clicked
            if screen_data.was_left_pressed(){
                game_event_manager.add_event(self.event.clone());    
            }
        }

        // Render button 
        texture_manager.render_ui_element_with_pos(button_texture, self.buffered_pos);

        // Render aperence values
        self.render_apearence(texture_manager);
    }
}